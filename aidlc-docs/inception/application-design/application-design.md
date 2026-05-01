# Application Design — MealPrep CLI/TUI

## Tech Stack

| Component | Choice |
|---|---|
| Language | Rust |
| TUI Framework | Ratatui |
| CLI Parsing | clap (nested subcommands) |
| Serialization | serde + serde_json |
| UUID | uuid crate |
| PBT | proptest |
| Build | cargo |

## Architecture Overview

8 components organized in a layered architecture:

```
+-------+
|  CLI  |  <-- clap argument parsing, entry point
+---+---+
    |
    v
+---+---+
|  App  |  <-- Ratatui TUI loop + orchestration
+---+---+
    |
    +----------+-----------+----------------+
    |          |           |                |
    v          v           v                v
+--------+ +--------+ +----------+  +---------+
| Recipe | | Meal   | | Grocery  |  | Storage |
| Store  | | Planner| | ListGen  |  |  (I/O)  |
+--------+ +--------+ +----------+  +---------+
    |          |            |
    v          v            v
+---------+ +---------+ +--------+
| Storage | | Scaling | | Models |
|         | | Util    | | (leaf) |
+---------+ +---------+ +--------+
```

## Components

| # | Component | Purpose | State |
|---|---|---|---|
| 1 | CLI | clap entry point, dispatches to App | Stateless |
| 2 | App | TUI state, event loop, orchestration | Stateful |
| 3 | RecipeStore | Recipe CRUD, UUID assignment | Stateful |
| 4 | MealPlanner | Weekly meal calendar management | Stateful |
| 5 | GroceryListGenerator | Consolidated grocery list generation | Stateless |
| 6 | ScalingUtil | Ingredient quantity scaling (pure) | Stateless |
| 7 | Storage | JSON file I/O abstraction | Stateless |
| 8 | Models | Shared data types (Recipe, MealPlan, etc.) | N/A |

## Data Files (UUID-indexed, separate)

| File | Contents | Linked By |
|---|---|---|
| `recipes.json` | `Vec<Recipe>` — each with UUID | Recipe UUID |
| `mealplan.json` | `MealPlan` — maps (Day, Slot) → Recipe UUID | Recipe UUID |
| `grocerylist.json` | `GroceryList` — items with checked state | Generated from plan |
| `config.json` | `Config` — household_size | N/A |

## Key Design Decisions

1. **Separate JSON files with UUID indexing** — recipes, meal plans, and grocery lists are stored in separate files. UUIDs link recipes across files.
2. **ScalingUtil as shared pure utility** — ingredient scaling is a stateless function, easily testable with PBT (invariant properties).
3. **App as single orchestrator** — no separate service layer needed for single-user MVP.
4. **Ratatui TUI** — interactive terminal UI with keyboard navigation, replacing simple CLI output.

## Detailed Artifacts

- [components.md](components.md) — Component definitions and responsibilities
- [component-methods.md](component-methods.md) — Method signatures and interfaces
- [services.md](services.md) — Service layer and orchestration patterns
- [component-dependency.md](component-dependency.md) — Dependency relationships and data flow
