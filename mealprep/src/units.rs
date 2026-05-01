use crate::models::Unit;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnitGroup {
    Weight,
    Volume,
    Count,
    Custom(String),
}

pub fn to_base_unit(quantity: f64, unit: &Unit) -> (f64, UnitGroup) {
    match unit {
        Unit::Grams => (quantity, UnitGroup::Weight),
        Unit::Kilograms => (quantity * 1000.0, UnitGroup::Weight),
        Unit::Ounces => (quantity * 28.3495, UnitGroup::Weight),
        Unit::Pounds => (quantity * 453.592, UnitGroup::Weight),
        Unit::Milliliters => (quantity, UnitGroup::Volume),
        Unit::Liters => (quantity * 1000.0, UnitGroup::Volume),
        Unit::Teaspoons => (quantity * 4.929, UnitGroup::Volume),
        Unit::Tablespoons => (quantity * 14.787, UnitGroup::Volume),
        Unit::Cups => (quantity * 236.588, UnitGroup::Volume),
        Unit::Pieces => (quantity, UnitGroup::Count),
        Unit::Custom(s) => (quantity, UnitGroup::Custom(s.clone())),
    }
}

pub fn from_base_unit(quantity: f64, group: &UnitGroup) -> (f64, Unit) {
    match group {
        UnitGroup::Weight => {
            if quantity >= 1000.0 {
                (quantity / 1000.0, Unit::Kilograms)
            } else {
                (quantity, Unit::Grams)
            }
        }
        UnitGroup::Volume => {
            if quantity >= 1000.0 {
                (quantity / 1000.0, Unit::Liters)
            } else if quantity >= 236.588 {
                (quantity / 236.588, Unit::Cups)
            } else {
                (quantity, Unit::Milliliters)
            }
        }
        UnitGroup::Count => (quantity, Unit::Pieces),
        UnitGroup::Custom(s) => (quantity, Unit::Custom(s.clone())),
    }
}

pub fn unit_group(unit: &Unit) -> UnitGroup {
    to_base_unit(1.0, unit).1
}
