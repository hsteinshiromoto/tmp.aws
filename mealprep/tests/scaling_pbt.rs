use mealprep::scaling;
use proptest::prelude::*;

// TP-01: Scaling round-trip
proptest! {
    #[test]
    fn scaling_roundtrip(
        quantity in 0.01f64..10000.0,
        recipe_servings in 1u32..100,
        household_size in 1u32..100,
    ) {
        let scaled = scaling::scale_quantity(quantity, recipe_servings, household_size);
        let restored = scaling::scale_quantity(scaled, household_size, recipe_servings);
        let diff = (restored - quantity).abs();
        prop_assert!(diff < 1e-6 * quantity.abs().max(1.0),
            "Round-trip failed: {} -> {} -> {}, diff={}", quantity, scaled, restored, diff);
    }
}

// TP-03: Proportionality invariant
proptest! {
    #[test]
    fn scaling_proportionality(
        q1 in 0.01f64..10000.0,
        q2 in 0.01f64..10000.0,
        recipe_servings in 1u32..100,
        household_size in 1u32..100,
    ) {
        let s1 = scaling::scale_quantity(q1, recipe_servings, household_size);
        let s2 = scaling::scale_quantity(q2, recipe_servings, household_size);
        let ratio_input = q1 / q2;
        let ratio_output = s1 / s2;
        let diff = (ratio_input - ratio_output).abs();
        prop_assert!(diff < 1e-9,
            "Proportionality failed: q1/q2={}, s1/s2={}", ratio_input, ratio_output);
    }
}

// TP-06: Identity scaling idempotence
proptest! {
    #[test]
    fn scaling_identity(
        quantity in 0.01f64..10000.0,
        servings in 1u32..100,
    ) {
        let result = scaling::scale_quantity(quantity, servings, servings);
        let diff = (result - quantity).abs();
        prop_assert!(diff < 1e-10,
            "Identity scaling failed: {} != {}", result, quantity);
    }
}
