use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub default_servings: u32,
    pub instructions: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: f64,
    pub unit: Unit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    Grams,
    Kilograms,
    Ounces,
    Pounds,
    Milliliters,
    Liters,
    Teaspoons,
    Tablespoons,
    Cups,
    Pieces,
    Custom(String),
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Grams => write!(f, "g"),
            Unit::Kilograms => write!(f, "kg"),
            Unit::Ounces => write!(f, "oz"),
            Unit::Pounds => write!(f, "lb"),
            Unit::Milliliters => write!(f, "ml"),
            Unit::Liters => write!(f, "L"),
            Unit::Teaspoons => write!(f, "tsp"),
            Unit::Tablespoons => write!(f, "tbsp"),
            Unit::Cups => write!(f, "cup"),
            Unit::Pieces => write!(f, "pcs"),
            Unit::Custom(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MealPlan {
    pub assignments: Vec<MealAssignment>,
    pub slots: Vec<String>,
}

impl Default for MealPlan {
    fn default() -> Self {
        Self {
            assignments: Vec::new(),
            slots: vec![
                "breakfast".to_string(),
                "lunch".to_string(),
                "dinner".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MealAssignment {
    pub day: Day,
    pub slot: String,
    pub recipe_id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    pub const ALL: [Day; 7] = [
        Day::Monday,
        Day::Tuesday,
        Day::Wednesday,
        Day::Thursday,
        Day::Friday,
        Day::Saturday,
        Day::Sunday,
    ];
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Day::Monday => write!(f, "Mon"),
            Day::Tuesday => write!(f, "Tue"),
            Day::Wednesday => write!(f, "Wed"),
            Day::Thursday => write!(f, "Thu"),
            Day::Friday => write!(f, "Fri"),
            Day::Saturday => write!(f, "Sat"),
            Day::Sunday => write!(f, "Sun"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroceryList {
    pub items: Vec<GroceryItem>,
    pub generated_at: String,
}

impl Default for GroceryList {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            generated_at: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroceryItem {
    pub name: String,
    pub quantity: f64,
    pub unit: Unit,
    pub checked: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub household_size: u32,
    pub data_dir: PathBuf,
    pub slots: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("mealprep");
        Self {
            household_size: 2,
            data_dir,
            slots: vec![
                "breakfast".to_string(),
                "lunch".to_string(),
                "dinner".to_string(),
            ],
        }
    }
}
