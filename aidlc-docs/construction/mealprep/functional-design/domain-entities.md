# Domain Entities — MealPrep

## Entity Relationship Overview

```
Config 1---1 HouseholdSize
Recipe 1---* Ingredient
MealPlan 1---* MealAssignment
MealAssignment *---1 Recipe (via UUID)
MealAssignment *---1 MealSlot
GroceryList 1---* GroceryItem
GroceryItem *---1 Ingredient (derived)
```

## Entities

### Recipe
| Field | Type | Description |
|---|---|---|
| id | Uuid | Unique identifier |
| name | String | Recipe name (required, non-empty, max 200 chars) |
| ingredients | Vec\<Ingredient\> | List of ingredients |
| default_servings | u32 | Default serving count (≥ 1) |
| instructions | String | Preparation instructions |

### Ingredient
| Field | Type | Description |
|---|---|---|
| name | String | Ingredient name (required, non-empty, max 100 chars) |
| quantity | f64 | Amount (> 0) |
| unit | Unit | Measurement unit |

### Unit (enum)
| Variant | Category | Description |
|---|---|---|
| Grams | Weight | Metric weight |
| Kilograms | Weight | Metric weight |
| Ounces | Weight | Imperial weight |
| Pounds | Weight | Imperial weight |
| Milliliters | Volume | Metric volume |
| Liters | Volume | Metric volume |
| Teaspoons | Volume | Cooking volume |
| Tablespoons | Volume | Cooking volume |
| Cups | Volume | Cooking volume |
| Pieces | Count | Discrete items |
| Custom(String) | Other | User-defined unit (no conversion) |

### MealPlan
| Field | Type | Description |
|---|---|---|
| assignments | Vec\<MealAssignment\> | All meal assignments for the week |
| slots | Vec\<String\> | Configured meal slot names (default: ["breakfast", "lunch", "dinner"]) |

### MealAssignment
| Field | Type | Description |
|---|---|---|
| day | Day | Day of week |
| slot | String | Meal slot name (must exist in MealPlan.slots) |
| recipe_id | Uuid | Reference to Recipe |

### Day (enum)
Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday

### GroceryList
| Field | Type | Description |
|---|---|---|
| items | Vec\<GroceryItem\> | Consolidated grocery items |
| generated_at | String | ISO 8601 timestamp of generation |

### GroceryItem
| Field | Type | Description |
|---|---|---|
| name | String | Ingredient name |
| quantity | f64 | Total quantity (scaled + merged) |
| unit | Unit | Measurement unit |
| checked | bool | Whether item has been purchased |

### Config
| Field | Type | Description |
|---|---|---|
| household_size | u32 | Number of servings to scale to (≥ 1, default: 2) |
| data_dir | PathBuf | Data directory (XDG-compliant default) |
| slots | Vec\<String\> | Configured meal slots (default: ["breakfast", "lunch", "dinner"]) |
