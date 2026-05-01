# Components

## 1. CLI (cli)
**Purpose**: Entry point. Parses commands via clap, dispatches to App.
**Responsibilities**:
- Define nested subcommand structure (`recipe`, `plan`, `grocery`, `config`)
- Parse and validate CLI arguments
- Invoke App methods and render output

## 2. App (app)
**Purpose**: TUI application state and Ratatui rendering loop.
**Responsibilities**:
- Manage TUI state (current view, selected items, input buffers)
- Handle keyboard events and navigation
- Delegate business operations to service layer
- Render views using Ratatui widgets

## 3. RecipeStore (recipe_store)
**Purpose**: CRUD operations for recipes.
**Responsibilities**:
- Add, list, get, delete recipes
- Assign UUID to each recipe on creation
- Persist recipes to `recipes.json` via Storage

## 4. MealPlanner (meal_planner)
**Purpose**: Manage weekly meal calendar.
**Responsibilities**:
- Assign recipe UUIDs to (day, meal_slot) pairs
- View, clear, replace meal assignments
- Persist meal plan to `mealplan.json` via Storage

## 5. GroceryListGenerator (grocery_list)
**Purpose**: Generate and manage consolidated grocery lists.
**Responsibilities**:
- Collect ingredients from all planned meals
- Scale quantities using ScalingUtil
- Merge identical ingredients and sum quantities
- Manage check-off state
- Persist grocery list to `grocerylist.json` via Storage

## 6. ScalingUtil (scaling)
**Purpose**: Shared utility for ingredient quantity scaling.
**Responsibilities**:
- Scale ingredient quantities proportionally: `quantity * (household_size / recipe_default_servings)`
- Stateless, pure function — ideal for PBT

## 7. Storage (storage)
**Purpose**: JSON file I/O abstraction.
**Responsibilities**:
- Read/write typed data to JSON files using serde
- Manage data directory location
- Handle file-not-found (return defaults) and write errors

## 8. Models (models)
**Purpose**: Shared data types.
**Responsibilities**:
- Define `Recipe`, `Ingredient`, `MealPlan`, `MealSlot`, `GroceryList`, `GroceryItem`, `Config`
- All models derive `Serialize`, `Deserialize`, `Clone`, `Debug`
- UUID fields for cross-file linking
