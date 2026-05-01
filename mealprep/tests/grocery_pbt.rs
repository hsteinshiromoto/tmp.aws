use mealprep::grocery;
use mealprep::models::*;
use mealprep::scaling;
use mealprep::units;
use proptest::prelude::*;
use std::collections::HashMap;

fn arb_unit_standard() -> impl Strategy<Value = Unit> {
    prop_oneof![
        Just(Unit::Grams),
        Just(Unit::Cups),
        Just(Unit::Pieces),
    ]
}

fn arb_ingredient() -> impl Strategy<Value = Ingredient> {
    ("[a-z]{3,8}", 0.1f64..100.0, arb_unit_standard()).prop_map(|(name, quantity, unit)| {
        Ingredient {
            name,
            quantity,
            unit,
        }
    })
}

// TP-04: Total quantity preserved after consolidation (algorithm-level test)
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn grocery_consolidation_preserves_total_quantity(
        ingredients in prop::collection::vec(arb_ingredient(), 1..6),
        recipe_servings in 1u32..10,
        household_size in 1u32..10,
    ) {
        // Compute expected totals by (lowercase_name, group) in base units
        let scaled = scaling::scale_ingredients(&ingredients, recipe_servings, household_size);
        let mut expected: HashMap<(String, String), f64> = HashMap::new();
        for ing in &scaled {
            let (base_qty, group) = units::to_base_unit(ing.quantity, &ing.unit);
            let key = (ing.name.to_lowercase(), format!("{:?}", group));
            *expected.entry(key).or_insert(0.0) += base_qty;
        }

        // Build a plan with one recipe containing these ingredients
        let recipe_id = uuid::Uuid::new_v4();
        let recipe = Recipe {
            id: recipe_id,
            name: "test".to_string(),
            ingredients: ingredients.clone(),
            default_servings: recipe_servings,
            instructions: String::new(),
        };
        let plan = MealPlan {
            assignments: vec![MealAssignment {
                day: Day::Monday,
                slot: "lunch".to_string(),
                recipe_id,
            }],
            slots: vec!["lunch".to_string()],
        };

        // Use generate_from_recipes which takes a slice
        let list = grocery::generate_from_recipes(&plan, &[recipe], household_size);

        // Verify: total base-unit quantity per (name, group) matches
        let mut actual: HashMap<(String, String), f64> = HashMap::new();
        for item in &list.items {
            let (base_qty, group) = units::to_base_unit(item.quantity, &item.unit);
            let key = (item.name.to_lowercase(), format!("{:?}", group));
            *actual.entry(key).or_insert(0.0) += base_qty;
        }

        for (key, exp_qty) in &expected {
            let act_qty = actual.get(key).copied().unwrap_or(0.0);
            let diff = (exp_qty - act_qty).abs();
            prop_assert!(diff < 1e-6 * exp_qty.abs().max(1.0),
                "Quantity mismatch for {:?}: expected={}, actual={}", key, exp_qty, act_qty);
        }
    }
}

// TP-05: Check-off toggle preserves item count
proptest! {
    #[test]
    fn grocery_check_off_toggle(
        items_count in 1usize..10,
        index in 0usize..10,
    ) {
        let items: Vec<GroceryItem> = (0..items_count)
            .map(|i| GroceryItem {
                name: format!("item{i}"),
                quantity: 1.0,
                unit: Unit::Pieces,
                checked: false,
            })
            .collect();
        let mut list = GroceryList {
            items,
            generated_at: String::new(),
        };

        let valid_index = index % items_count;
        grocery::check_off(&mut list, valid_index).unwrap();
        prop_assert!(list.items[valid_index].checked);

        grocery::uncheck(&mut list, valid_index).unwrap();
        prop_assert!(!list.items[valid_index].checked);

        // Item count unchanged
        prop_assert_eq!(list.items.len(), items_count);
    }
}
