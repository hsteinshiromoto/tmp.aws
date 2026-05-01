# Component Dependencies

## Dependency Matrix

| Component | Depends On |
|---|---|
| CLI | App |
| App | RecipeStore, MealPlanner, GroceryListGenerator, Storage, Models |
| RecipeStore | Storage, Models |
| MealPlanner | Storage, Models |
| GroceryListGenerator | ScalingUtil, Models |
| ScalingUtil | Models (Ingredient only) |
| Storage | Models (via serde generics) |
| Models | (none — leaf dependency) |

## Communication Patterns

- **All components are synchronous** — no async needed for local file I/O
- **Ownership**: App owns RecipeStore, MealPlanner. GroceryListGenerator is stateless (called with references).
- **ScalingUtil**: Pure functions, no state. Called by GroceryListGenerator.
- **Storage**: Static methods. No instance state. Called by RecipeStore, MealPlanner, and App (for config/grocery persistence).

## Dependency Diagram

```
+-------+
|  CLI  |
+---+---+
    |
    v
+---+---+
|  App  |
+---+---+
    |
    +----------+-----------+----------------+
    |          |           |                |
    v          v           v                v
+--------+ +--------+ +----------+  +---------+
| Recipe | | Meal   | | Grocery  |  | Storage |
| Store  | | Planner| | ListGen  |  |         |
+---+----+ +---+----+ +----+-----+  +---------+
    |          |            |
    +----+-----+       +----+----+
         |             |         |
         v             v         v
    +---------+   +---------+ +--------+
    | Storage |   | Scaling | | Models |
    +---------+   | Util    | +--------+
                  +---------+
```
