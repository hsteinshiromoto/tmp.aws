use crate::models::{GroceryItem, GroceryList, MealPlan, Recipe};
use crate::recipe_store::RecipeStore;
use crate::scaling;
use crate::units::{self, UnitGroup};
use anyhow::{bail, Result};
use std::collections::HashMap;

pub fn generate(plan: &MealPlan, recipes: &RecipeStore, household_size: u32) -> GroceryList {
    let recipe_list: Vec<&Recipe> = plan
        .assignments
        .iter()
        .filter_map(|a| recipes.get(a.recipe_id))
        .collect();
    generate_inner(plan, &recipe_list, household_size)
}

pub fn generate_from_recipes(plan: &MealPlan, recipes: &[Recipe], household_size: u32) -> GroceryList {
    let recipe_refs: Vec<&Recipe> = plan
        .assignments
        .iter()
        .filter_map(|a| recipes.iter().find(|r| r.id == a.recipe_id))
        .collect();
    generate_inner(plan, &recipe_refs, household_size)
}

fn generate_inner(plan: &MealPlan, recipe_refs: &[&Recipe], household_size: u32) -> GroceryList {
    let _ = plan; // plan already used to resolve recipes above
    let mut merged: HashMap<(String, UnitGroup), f64> = HashMap::new();

    for recipe in recipe_refs {
        let scaled = scaling::scale_ingredients(
            &recipe.ingredients,
            recipe.default_servings,
            household_size,
        );
        for ing in &scaled {
            let (base_qty, group) = units::to_base_unit(ing.quantity, &ing.unit);
            let key = (ing.name.to_lowercase(), group);
            *merged.entry(key).or_insert(0.0) += base_qty;
        }
    }

    let mut items: Vec<GroceryItem> = merged
        .into_iter()
        .map(|((name, group), total_qty)| {
            let (display_qty, display_unit) = units::from_base_unit(total_qty, &group);
            GroceryItem {
                name,
                quantity: display_qty,
                unit: display_unit,
                checked: false,
            }
        })
        .collect();

    items.sort_by(|a, b| a.name.cmp(&b.name));

    GroceryList {
        items,
        generated_at: chrono::Utc::now().to_rfc3339(),
    }
}

pub fn check_off(list: &mut GroceryList, item_index: usize) -> Result<()> {
    match list.items.get_mut(item_index) {
        Some(item) => {
            item.checked = true;
            Ok(())
        }
        None => bail!("Item index out of range"),
    }
}

pub fn uncheck(list: &mut GroceryList, item_index: usize) -> Result<()> {
    match list.items.get_mut(item_index) {
        Some(item) => {
            item.checked = false;
            Ok(())
        }
        None => bail!("Item index out of range"),
    }
}

pub fn remaining(list: &GroceryList) -> Vec<&GroceryItem> {
    list.items.iter().filter(|i| !i.checked).collect()
}
