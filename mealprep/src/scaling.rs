use crate::models::Ingredient;

pub fn scale_quantity(quantity: f64, recipe_servings: u32, household_size: u32) -> f64 {
    quantity * (household_size as f64 / recipe_servings as f64)
}

pub fn scale_ingredients(
    ingredients: &[Ingredient],
    recipe_servings: u32,
    household_size: u32,
) -> Vec<Ingredient> {
    ingredients
        .iter()
        .map(|ing| Ingredient {
            name: ing.name.clone(),
            quantity: scale_quantity(ing.quantity, recipe_servings, household_size),
            unit: ing.unit.clone(),
        })
        .collect()
}
