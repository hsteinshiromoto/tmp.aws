# Services

## Service Layer: App

The `App` struct serves as the single orchestration layer. Given the MVP scope (single-user CLI/TUI, no network), a dedicated service layer is unnecessary. `App` coordinates between components.

### Orchestration Patterns

**Recipe Flow**: CLI/TUI input → App → RecipeStore → Storage
**Meal Planning Flow**: CLI/TUI input → App → MealPlanner (validates recipe exists via RecipeStore) → Storage
**Grocery Generation Flow**: CLI/TUI input → App → GroceryListGenerator (reads MealPlanner + RecipeStore, uses ScalingUtil) → Storage
**Config Flow**: CLI/TUI input → App → Storage (read/write config.json for household_size)

### Data Flow Summary

```
User Input
    |
    v
  App (orchestrator)
    |
    +---> RecipeStore ---> Storage (recipes.json)
    |
    +---> MealPlanner ---> Storage (mealplan.json)
    |         |
    |         +---> validates recipe IDs via RecipeStore
    |
    +---> GroceryListGenerator ---> Storage (grocerylist.json)
              |
              +---> reads MealPlanner.get_plan()
              +---> reads RecipeStore.get() per recipe
              +---> calls ScalingUtil.scale_ingredients()
```
