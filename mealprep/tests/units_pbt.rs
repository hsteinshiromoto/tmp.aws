use mealprep::models::Unit;
use mealprep::units;
use proptest::prelude::*;

fn arb_standard_unit() -> impl Strategy<Value = Unit> {
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
    ]
}

// TP-02: Unit conversion round-trip (quantity preserved)
proptest! {
    #[test]
    fn unit_conversion_quantity_preserved(
        quantity in 0.01f64..10000.0,
        unit in arb_standard_unit(),
    ) {
        let (base_qty, group) = units::to_base_unit(quantity, &unit);
        let (back_qty, _back_unit) = units::from_base_unit(base_qty, &group);
        // Convert back_qty to base again to compare
        let (base_again, _) = units::to_base_unit(back_qty, &_back_unit);
        let diff = (base_qty - base_again).abs();
        prop_assert!(diff < 1e-6 * base_qty.abs().max(1.0),
            "Round-trip quantity mismatch: base={}, restored_base={}, diff={}", base_qty, base_again, diff);
    }
}
