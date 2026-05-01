# Code Generation Summary — MealPrep

## Files Generated

### Source Files (src/)
| File | Purpose | Lines |
|---|---|---|
| main.rs | Entry point, Ratatui terminal setup, event loop | ~60 |
| lib.rs | Library crate exposing public modules for tests | ~7 |
| models.rs | Domain entities (Recipe, Ingredient, Unit, MealPlan, Day, GroceryList, Config) | ~163 |
| storage.rs | Generic JSON load/save with XDG data dir | ~25 |
| units.rs | Unit conversion (to_base_unit, from_base_unit) | ~52 |
| scaling.rs | Ingredient quantity scaling (pure functions) | ~20 |
| recipe_store.rs | Recipe CRUD with validation and UUID assignment | ~72 |
| meal_planner.rs | Weekly meal calendar management | ~74 |
| grocery.rs | Grocery list generation, consolidation, check-off | ~88 |
| app.rs | TUI state, event handling, orchestration | ~338 |
| ui.rs | Ratatui rendering (recipes, plan, grocery, settings views) | ~266 |

### Test Files (tests/)
| File | Properties Tested | Tests |
|---|---|---|
| models_pbt.rs | TP-07, TP-08, TP-09, TP-10 (serde round-trips) | 4 |
| scaling_pbt.rs | TP-01, TP-03, TP-06 (round-trip, proportionality, identity) | 3 |
| units_pbt.rs | TP-02 (unit conversion round-trip) | 1 |
| grocery_pbt.rs | TP-04, TP-05 (quantity preservation, check-off toggle) | 2 |

### Configuration
| File | Purpose |
|---|---|
| Cargo.toml | Dependencies and project metadata |
| README.md | Usage instructions |

## Test Results
- **10 PBT tests**: All passing
- **0 failures**
