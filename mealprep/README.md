# MealPrep

Weekly meal planning TUI with smart grocery lists.

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run
```

## Navigation

| Key | Action |
|---|---|
| `1` | Recipes view |
| `2` | Meal Plan view |
| `3` | Grocery List view |
| `4` | Settings view |
| `q` | Quit |
| `j`/`k` or `↑`/`↓` | Navigate lists |
| `Ctrl+C` | Force quit |

### Recipes View
- `a` — Add new recipe
- `d` — Delete selected recipe

### Recipe Add Form
- `Tab` / `Shift+Tab` — Navigate fields
- `Enter` — Submit (on last field) or next field
- `Esc` — Cancel
- Ingredients format: `qty unit name; qty unit name; ...` (e.g., `2 cups flour; 100 g sugar`)

### Meal Plan View
- `a` — Assign a recipe to a meal slot
- In assign mode: `←`/`→` day, `↑`/`↓` slot, `j`/`k` recipe, `Enter` assign, `x` clear

### Grocery List View
- `g` — Generate grocery list from current meal plan
- `Space` — Toggle check-off on selected item

### Settings View
- `+`/`-` — Adjust household size

## Data Storage

Data is stored in XDG-compliant directories (e.g., `~/.local/share/mealprep/` on Linux):
- `recipes.json` — Recipe library
- `mealplan.json` — Weekly meal assignments
- `grocerylist.json` — Generated grocery list with check-off state
- `config.json` — Settings (household size, meal slots)

## Tests

```bash
cargo test
```

Includes property-based tests (proptest) for:
- Serde round-trips (Recipe, MealPlan, GroceryList, Config)
- Scaling round-trip, proportionality, identity
- Unit conversion round-trip
- Grocery consolidation quantity preservation
