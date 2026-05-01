# Code Generation Plan — MealPrep

## Unit Context
- **Unit**: mealprep (single unit)
- **Language**: Rust
- **TUI**: Ratatui
- **CLI**: clap
- **Serialization**: serde + serde_json
- **PBT**: proptest
- **Workspace Root**: /Users/hsteinshiromoto/Projects/amazon/2026-05-21

## Project Structure
```
mealprep/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, clap CLI + TUI launch
│   ├── models.rs            # Domain entities (Recipe, Ingredient, Unit, MealPlan, etc.)
│   ├── storage.rs           # JSON file I/O (load/save generic)
│   ├── scaling.rs           # ScalingUtil (scale_quantity, scale_ingredients)
│   ├── units.rs             # Unit conversion (to_base_unit, from_base_unit)
│   ├── recipe_store.rs      # RecipeStore (CRUD)
│   ├── meal_planner.rs      # MealPlanner (weekly calendar)
│   ├── grocery.rs           # GroceryListGenerator (generate, check_off, remaining)
│   ├── app.rs               # App struct (TUI state + orchestration)
│   └── ui.rs                # Ratatui rendering (views for recipes, plan, grocery)
└── tests/
    ├── models_pbt.rs         # PBT: serde round-trips (TP-07..10)
    ├── scaling_pbt.rs        # PBT: scaling properties (TP-01, TP-03, TP-06)
    ├── units_pbt.rs          # PBT: unit conversion round-trip (TP-02)
    └── grocery_pbt.rs        # PBT: consolidation invariants (TP-04, TP-05)
```

## Generation Steps

### Step 1: Project Setup
- [x] Create `mealprep/Cargo.toml` with dependencies (ratatui, crossterm, clap, serde, serde_json, uuid, dirs, chrono, proptest)
- [x] Create `mealprep/src/main.rs` with clap CLI skeleton and TUI entry point

### Step 2: Domain Models
- [x] Create `mealprep/src/models.rs` — all domain entities per domain-entities.md
- [x] Derive Serialize, Deserialize, Clone, Debug, PartialEq on all types
- [x] Implement Unit enum with all variants including Custom(String)
- [x] Implement Day enum with Display trait
- [x] Implement Default for Config and MealPlan

### Step 3: Storage Layer
- [x] Create `mealprep/src/storage.rs` — generic load/save functions
- [x] Implement `data_dir()` using `dirs` crate (XDG-compliant)
- [x] Handle file-not-found gracefully (return Default)

### Step 4: Unit Conversion
- [x] Create `mealprep/src/units.rs` — to_base_unit, from_base_unit per ALG-02
- [x] Implement UnitGroup enum (Weight, Volume, Count, Custom)
- [x] Implement conversion factors per BR-08

### Step 5: Scaling Utility
- [x] Create `mealprep/src/scaling.rs` — scale_quantity, scale_ingredients per ALG-01
- [x] Pure functions, no side effects

### Step 6: Recipe Store
- [x] Create `mealprep/src/recipe_store.rs` — RecipeStore struct with CRUD methods
- [x] Input validation per BR-01, BR-02, BR-03, BR-04, BR-12
- [x] UUID assignment on add
- [x] Persist via Storage on mutations

### Step 7: Meal Planner
- [x] Create `mealprep/src/meal_planner.rs` — MealPlanner struct
- [x] Implement set_meal, clear_meal, get_plan, get_meal per component-methods.md
- [x] Validate slot exists in configured slots (BR-05, BR-06)
- [x] Persist via Storage on mutations

### Step 8: Grocery List Generator
- [x] Create `mealprep/src/grocery.rs` — generate, check_off, uncheck, remaining per ALG-03
- [x] Implement ingredient merging with unit conversion per BR-09
- [x] Lifecycle per BR-10 (explicit generation, persistent check-off)

### Step 9: App Orchestration
- [x] Create `mealprep/src/app.rs` — App struct holding RecipeStore, MealPlanner, GroceryList, Config
- [x] Implement recipe deletion with warn+confirm per BR-11
- [x] TUI state management (current view, selection, input mode)
- [x] Event handling (keyboard input dispatch)

### Step 10: TUI Rendering
- [x] Create `mealprep/src/ui.rs` — Ratatui rendering functions
- [x] Recipe list view, recipe detail/add form
- [x] Weekly meal plan grid view
- [x] Grocery list view with check-off toggle
- [x] Config/settings view (household size, meal slots)
- [x] Navigation bar and help overlay

### Step 11: PBT Tests — Models
- [x] Create `mealprep/tests/models_pbt.rs`
- [x] TP-07: Recipe serde round-trip
- [x] TP-08: MealPlan serde round-trip
- [x] TP-09: GroceryList serde round-trip
- [x] TP-10: Config serde round-trip
- [x] Custom proptest strategies for domain types (PBT-07)

### Step 12: PBT Tests — Scaling
- [x] Create `mealprep/tests/scaling_pbt.rs`
- [x] TP-01: Scaling round-trip (scale up then back ≈ original)
- [x] TP-03: Proportionality invariant
- [x] TP-06: Identity scaling idempotence (household == recipe servings)

### Step 13: PBT Tests — Unit Conversion
- [x] Create `mealprep/tests/units_pbt.rs`
- [x] TP-02: Unit conversion round-trip (to_base then from_base ≈ original)

### Step 14: PBT Tests — Grocery Consolidation
- [x] Create `mealprep/tests/grocery_pbt.rs`
- [x] TP-04: Total quantity preserved after consolidation
- [x] TP-05: Item count invariant (unique name+group pairs)

### Step 15: Documentation
- [x] Create `mealprep/README.md` with usage instructions
- [x] Create `aidlc-docs/construction/mealprep/code/code-summary.md` with file listing and description

## Story Traceability
| Requirement | Steps |
|---|---|
| FR-01: Recipe Management | Steps 2, 6, 9, 10 |
| FR-02: Weekly Meal Calendar | Steps 2, 7, 9, 10 |
| FR-03: Grocery List Generation | Steps 4, 5, 8, 9, 10 |
| FR-04: Grocery List Check-Off | Steps 8, 10 |
| FR-05: Serving Size Adjustment | Steps 5, 8, 9, 10 |
| NFR-04: Security (SECURITY-05) | Steps 6, 7 |
| NFR-05: Testability (PBT) | Steps 11, 12, 13, 14 |
