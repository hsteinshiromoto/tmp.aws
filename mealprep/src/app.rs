use crate::grocery;
use crate::meal_planner::MealPlanner;
use crate::models::{Config, Day, GroceryList, Ingredient, Recipe, Unit};
use crate::recipe_store::RecipeStore;
use crate::storage;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    Recipes,
    RecipeAdd,
    MealPlan,
    MealAssign,
    Grocery,
    Settings,
    ConfirmDelete,
}

pub struct App {
    pub should_quit: bool,
    pub view: View,
    pub recipes: RecipeStore,
    pub planner: MealPlanner,
    pub grocery_list: GroceryList,
    pub config: Config,
    pub selected_index: usize,
    pub input_buffer: String,
    pub input_fields: Vec<String>,
    pub input_field_index: usize,
    pub status_msg: String,
    pub selected_day: usize,
    pub selected_slot: usize,
    pub pending_delete: Option<Uuid>,
}

impl App {
    pub fn new() -> Result<Self> {
        let config: Config = storage::load(&Config::default().data_dir.join("config.json"))
            .unwrap_or_default();
        let recipes = RecipeStore::load(config.data_dir.join("recipes.json"))?;
        let planner = MealPlanner::load(config.data_dir.join("mealplan.json"), &config.slots)?;
        let grocery_list: GroceryList =
            storage::load(&config.data_dir.join("grocerylist.json")).unwrap_or_default();

        Ok(Self {
            should_quit: false,
            view: View::Recipes,
            recipes,
            planner,
            grocery_list,
            config,
            selected_index: 0,
            input_buffer: String::new(),
            input_fields: Vec::new(),
            input_field_index: 0,
            status_msg: String::new(),
            selected_day: 0,
            selected_slot: 0,
            pending_delete: None,
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match self.view {
            View::ConfirmDelete => self.handle_confirm_delete(key),
            View::RecipeAdd => self.handle_recipe_add(key),
            View::MealAssign => self.handle_meal_assign(key),
            _ => self.handle_navigation(key),
        }
    }

    fn handle_navigation(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('1') => self.view = View::Recipes,
            KeyCode::Char('2') => self.view = View::MealPlan,
            KeyCode::Char('3') => self.view = View::Grocery,
            KeyCode::Char('4') => self.view = View::Settings,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_index += 1;
            }
            KeyCode::Char('a') if self.view == View::Recipes => {
                self.view = View::RecipeAdd;
                self.input_fields = vec![String::new(); 4]; // name, servings, ingredients, instructions
                self.input_field_index = 0;
            }
            KeyCode::Char('d') if self.view == View::Recipes => {
                self.try_delete_recipe()?;
            }
            KeyCode::Char('a') if self.view == View::MealPlan => {
                self.view = View::MealAssign;
                self.selected_index = 0;
            }
            KeyCode::Char('g') if self.view == View::Grocery => {
                self.generate_grocery_list()?;
            }
            KeyCode::Char(' ') if self.view == View::Grocery => {
                self.toggle_grocery_item()?;
            }
            KeyCode::Char('+') if self.view == View::Settings => {
                self.config.household_size = self.config.household_size.saturating_add(1);
                self.save_config()?;
            }
            KeyCode::Char('-') if self.view == View::Settings => {
                if self.config.household_size > 1 {
                    self.config.household_size -= 1;
                    self.save_config()?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_recipe_add(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.view = View::Recipes,
            KeyCode::Tab => {
                self.input_field_index = (self.input_field_index + 1) % self.input_fields.len();
            }
            KeyCode::BackTab => {
                self.input_field_index = if self.input_field_index == 0 {
                    self.input_fields.len() - 1
                } else {
                    self.input_field_index - 1
                };
            }
            KeyCode::Enter => {
                if self.input_field_index == self.input_fields.len() - 1 {
                    self.submit_recipe()?;
                } else {
                    self.input_field_index += 1;
                }
            }
            KeyCode::Char(c) => {
                self.input_fields[self.input_field_index].push(c);
            }
            KeyCode::Backspace => {
                self.input_fields[self.input_field_index].pop();
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_meal_assign(&mut self, key: KeyEvent) -> Result<()> {
        let recipe_count = self.recipes.list().len();
        match key.code {
            KeyCode::Esc => self.view = View::MealPlan,
            KeyCode::Left => {
                self.selected_day = self.selected_day.checked_sub(1).unwrap_or(6);
            }
            KeyCode::Right => {
                self.selected_day = (self.selected_day + 1) % 7;
            }
            KeyCode::Up => {
                let slot_count = self.planner.slots().len();
                self.selected_slot = self.selected_slot.checked_sub(1).unwrap_or(slot_count.saturating_sub(1));
            }
            KeyCode::Down => {
                let slot_count = self.planner.slots().len();
                self.selected_slot = (self.selected_slot + 1) % slot_count;
            }
            KeyCode::Enter if recipe_count > 0 => {
                let day = Day::ALL[self.selected_day];
                let slot = &self.planner.slots()[self.selected_slot].clone();
                let recipe = &self.recipes.list()[self.selected_index % recipe_count];
                let recipe_id = recipe.id;
                self.planner.set_meal(day, slot, recipe_id)?;
                self.status_msg = format!("Assigned to {} {}", day, slot);
                self.view = View::MealPlan;
            }
            KeyCode::Char('j') => {
                self.selected_index = (self.selected_index + 1) % recipe_count.max(1);
            }
            KeyCode::Char('k') => {
                self.selected_index = self.selected_index.checked_sub(1).unwrap_or(recipe_count.saturating_sub(1));
            }
            KeyCode::Delete | KeyCode::Char('x') => {
                let day = Day::ALL[self.selected_day];
                let slot = &self.planner.slots()[self.selected_slot].clone();
                self.planner.clear_meal(day, slot)?;
                self.status_msg = "Meal cleared".to_string();
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_confirm_delete(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Some(id) = self.pending_delete.take() {
                    self.planner.remove_recipe(id)?;
                    self.recipes.delete(id)?;
                    self.status_msg = "Recipe deleted".to_string();
                }
                self.view = View::Recipes;
            }
            _ => {
                self.pending_delete = None;
                self.status_msg = "Deletion cancelled".to_string();
                self.view = View::Recipes;
            }
        }
        Ok(())
    }

    fn try_delete_recipe(&mut self) -> Result<()> {
        let recipes = self.recipes.list();
        if recipes.is_empty() {
            return Ok(());
        }
        let idx = self.selected_index % recipes.len();
        let recipe_id = recipes[idx].id;
        let assignments = self.planner.assignments_for_recipe(recipe_id);
        if assignments.is_empty() {
            self.recipes.delete(recipe_id)?;
            self.status_msg = "Recipe deleted".to_string();
        } else {
            self.pending_delete = Some(recipe_id);
            self.status_msg = format!(
                "Recipe used in {} meal(s). Press 'y' to confirm delete, any other key to cancel.",
                assignments.len()
            );
            self.view = View::ConfirmDelete;
        }
        Ok(())
    }

    fn submit_recipe(&mut self) -> Result<()> {
        let name = self.input_fields[0].clone();
        let servings: u32 = self.input_fields[1].parse().unwrap_or(1).max(1);
        let ingredients = parse_ingredients(&self.input_fields[2]);
        let instructions = self.input_fields[3].clone();

        let recipe = Recipe {
            id: Uuid::nil(),
            name,
            ingredients,
            default_servings: servings,
            instructions,
        };

        match self.recipes.add(recipe) {
            Ok(_) => self.status_msg = "Recipe added".to_string(),
            Err(e) => self.status_msg = format!("Error: {e}"),
        }
        self.view = View::Recipes;
        Ok(())
    }

    fn generate_grocery_list(&mut self) -> Result<()> {
        self.grocery_list = grocery::generate(
            self.planner.get_plan(),
            &self.recipes,
            self.config.household_size,
        );
        storage::save(
            &self.config.data_dir.join("grocerylist.json"),
            &self.grocery_list,
        )?;
        self.status_msg = format!("Grocery list generated ({} items)", self.grocery_list.items.len());
        Ok(())
    }

    fn toggle_grocery_item(&mut self) -> Result<()> {
        if self.grocery_list.items.is_empty() {
            return Ok(());
        }
        let idx = self.selected_index % self.grocery_list.items.len();
        let item = &mut self.grocery_list.items[idx];
        item.checked = !item.checked;
        storage::save(
            &self.config.data_dir.join("grocerylist.json"),
            &self.grocery_list,
        )?;
        Ok(())
    }

    fn save_config(&self) -> Result<()> {
        storage::save(&self.config.data_dir.join("config.json"), &self.config)
    }
}

fn parse_ingredients(input: &str) -> Vec<Ingredient> {
    input
        .split(';')
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                return None;
            }
            let parts: Vec<&str> = s.splitn(3, ' ').collect();
            if parts.len() < 2 {
                return Some(Ingredient {
                    name: s.to_string(),
                    quantity: 1.0,
                    unit: Unit::Pieces,
                });
            }
            let quantity = parts[0].parse::<f64>().unwrap_or(1.0);
            let (unit, name) = if parts.len() == 3 {
                (parse_unit(parts[1]), parts[2].to_string())
            } else {
                (Unit::Pieces, parts[1].to_string())
            };
            Some(Ingredient {
                name,
                quantity,
                unit,
            })
        })
        .collect()
}

fn parse_unit(s: &str) -> Unit {
    match s.to_lowercase().as_str() {
        "g" | "grams" | "gram" => Unit::Grams,
        "kg" | "kilograms" | "kilogram" => Unit::Kilograms,
        "oz" | "ounces" | "ounce" => Unit::Ounces,
        "lb" | "lbs" | "pounds" | "pound" => Unit::Pounds,
        "ml" | "milliliters" | "milliliter" => Unit::Milliliters,
        "l" | "liters" | "liter" => Unit::Liters,
        "tsp" | "teaspoon" | "teaspoons" => Unit::Teaspoons,
        "tbsp" | "tablespoon" | "tablespoons" => Unit::Tablespoons,
        "cup" | "cups" => Unit::Cups,
        "pcs" | "pieces" | "piece" => Unit::Pieces,
        other => Unit::Custom(other.to_string()),
    }
}
