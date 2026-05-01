use crate::models::Recipe;
use crate::storage;
use anyhow::{bail, Result};
use std::path::PathBuf;
use uuid::Uuid;

pub struct RecipeStore {
    recipes: Vec<Recipe>,
    path: PathBuf,
}

impl RecipeStore {
    pub fn load(path: PathBuf) -> Result<Self> {
        let recipes: Vec<Recipe> = storage::load(&path)?;
        Ok(Self { recipes, path })
    }

    fn save(&self) -> Result<()> {
        storage::save(&self.path, &self.recipes)
    }

    pub fn add(&mut self, mut recipe: Recipe) -> Result<Uuid> {
        let name = recipe.name.trim().to_string();
        if name.is_empty() {
            bail!("Recipe name cannot be empty");
        }
        if name.len() > 200 {
            bail!("Recipe name exceeds 200 characters");
        }
        if recipe.default_servings < 1 {
            bail!("Default servings must be at least 1");
        }
        for ing in &recipe.ingredients {
            let ing_name = ing.name.trim();
            if ing_name.is_empty() {
                bail!("Ingredient name cannot be empty");
            }
            if ing_name.len() > 100 {
                bail!("Ingredient name exceeds 100 characters");
            }
            if ing.quantity <= 0.0 || !ing.quantity.is_finite() {
                bail!("Ingredient quantity must be a positive finite number");
            }
        }
        recipe.name = name;
        recipe.ingredients.iter_mut().for_each(|ing| {
            ing.name = ing.name.trim().to_string();
        });
        recipe.id = Uuid::new_v4();
        let id = recipe.id;
        self.recipes.push(recipe);
        self.save()?;
        Ok(id)
    }

    pub fn list(&self) -> &[Recipe] {
        &self.recipes
    }

    pub fn get(&self, id: Uuid) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.id == id)
    }

    pub fn delete(&mut self, id: Uuid) -> Result<()> {
        let len = self.recipes.len();
        self.recipes.retain(|r| r.id != id);
        if self.recipes.len() == len {
            bail!("Recipe not found");
        }
        self.save()
    }
}
