use mealprep::models::*;
use proptest::prelude::*;

fn arb_unit() -> impl Strategy<Value = Unit> {
    prop_oneof![
        Just(Unit::Grams),
        Just(Unit::Kilograms),
        Just(Unit::Ounces),
        Just(Unit::Pounds),
        Just(Unit::Milliliters),
        Just(Unit::Liters),
        Just(Unit::Teaspoons),
        Just(Unit::Tablespoons),
        Just(Unit::Cups),
        Just(Unit::Pieces),
        "[a-z]{1,10}".prop_map(Unit::Custom),
    ]
}

fn arb_ingredient() -> impl Strategy<Value = Ingredient> {
    ("[a-z ]{1,20}", 0.1f64..1000.0, arb_unit()).prop_map(|(name, quantity, unit)| Ingredient {
        name,
        quantity,
        unit,
    })
}

fn arb_recipe() -> impl Strategy<Value = Recipe> {
    (
        "[a-zA-Z ]{1,50}",
        prop::collection::vec(arb_ingredient(), 0..5),
        1u32..20,
        "[a-zA-Z ]{0,100}",
    )
        .prop_map(|(name, ingredients, default_servings, instructions)| Recipe {
            id: uuid::Uuid::new_v4(),
            name,
            ingredients,
            default_servings,
            instructions,
        })
}

fn arb_day() -> impl Strategy<Value = Day> {
    prop_oneof![
        Just(Day::Monday),
        Just(Day::Tuesday),
        Just(Day::Wednesday),
        Just(Day::Thursday),
        Just(Day::Friday),
        Just(Day::Saturday),
        Just(Day::Sunday),
    ]
}

fn arb_meal_assignment() -> impl Strategy<Value = MealAssignment> {
    (arb_day(), "(breakfast|lunch|dinner)").prop_map(|(day, slot)| MealAssignment {
        day,
        slot,
        recipe_id: uuid::Uuid::new_v4(),
    })
}

fn arb_meal_plan() -> impl Strategy<Value = MealPlan> {
    prop::collection::vec(arb_meal_assignment(), 0..10).prop_map(|assignments| MealPlan {
        assignments,
        slots: vec![
            "breakfast".to_string(),
            "lunch".to_string(),
            "dinner".to_string(),
        ],
    })
}

fn arb_grocery_item() -> impl Strategy<Value = GroceryItem> {
    ("[a-z ]{1,20}", 0.1f64..1000.0, arb_unit(), any::<bool>()).prop_map(
        |(name, quantity, unit, checked)| GroceryItem {
            name,
            quantity,
            unit,
            checked,
        },
    )
}

fn arb_grocery_list() -> impl Strategy<Value = GroceryList> {
    prop::collection::vec(arb_grocery_item(), 0..10).prop_map(|items| GroceryList {
        items,
        generated_at: "2026-01-01T00:00:00Z".to_string(),
    })
}

fn arb_config() -> impl Strategy<Value = Config> {
    (1u32..20,).prop_map(|(household_size,)| Config {
        household_size,
        ..Config::default()
    })
}

// TP-07: Recipe serde round-trip
proptest! {
    #[test]
    fn recipe_serde_roundtrip(recipe in arb_recipe()) {
        let json = serde_json::to_string(&recipe).unwrap();
        let deserialized: Recipe = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(&recipe.id, &deserialized.id);
        prop_assert_eq!(&recipe.name, &deserialized.name);
        prop_assert_eq!(recipe.ingredients.len(), deserialized.ingredients.len());
        for (a, b) in recipe.ingredients.iter().zip(deserialized.ingredients.iter()) {
            prop_assert_eq!(&a.name, &b.name);
            prop_assert!((a.quantity - b.quantity).abs() < 1e-10);
            prop_assert_eq!(&a.unit, &b.unit);
        }
        prop_assert_eq!(recipe.default_servings, deserialized.default_servings);
        prop_assert_eq!(&recipe.instructions, &deserialized.instructions);
    }
}

// TP-08: MealPlan serde round-trip
proptest! {
    #[test]
    fn meal_plan_serde_roundtrip(plan in arb_meal_plan()) {
        let json = serde_json::to_string(&plan).unwrap();
        let deserialized: MealPlan = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(plan, deserialized);
    }
}

// TP-09: GroceryList serde round-trip
proptest! {
    #[test]
    fn grocery_list_serde_roundtrip(list in arb_grocery_list()) {
        let json = serde_json::to_string(&list).unwrap();
        let deserialized: GroceryList = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(list.items.len(), deserialized.items.len());
        for (a, b) in list.items.iter().zip(deserialized.items.iter()) {
            prop_assert_eq!(&a.name, &b.name);
            prop_assert!((a.quantity - b.quantity).abs() < 1e-10);
            prop_assert_eq!(&a.unit, &b.unit);
            prop_assert_eq!(a.checked, b.checked);
        }
        prop_assert_eq!(&list.generated_at, &deserialized.generated_at);
    }
}

// TP-10: Config serde round-trip
proptest! {
    #[test]
    fn config_serde_roundtrip(config in arb_config()) {
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(config, deserialized);
    }
}
