# Integration Test Instructions

## Purpose
Test interactions between components (RecipeStore ↔ MealPlanner ↔ GroceryListGenerator ↔ Storage) to ensure they work together correctly with real JSON file I/O.

## Test Scenarios

### Scenario 1: Recipe → Meal Plan → Grocery List Pipeline
- **Description**: Add recipes, assign to meal plan, generate grocery list, verify consolidation
- **Setup**: Clean data directory (delete or use temp dir)
- **Steps**:
  1. Run `cargo run`, add 2+ recipes with overlapping ingredients
  2. Assign recipes to different meal slots
  3. Generate grocery list
  4. Verify ingredients are merged and scaled correctly
- **Expected**: Grocery list shows consolidated quantities, scaled by household size

### Scenario 2: Recipe Deletion with Meal Plan References
- **Description**: Delete a recipe that is assigned in the meal plan
- **Steps**:
  1. Add a recipe and assign it to a meal slot
  2. Attempt to delete the recipe
  3. Verify warning message shows affected meal slots
  4. Confirm deletion
  5. Verify recipe removed from both recipe store and meal plan
- **Expected**: Warning displayed, cascade deletion on confirm

### Scenario 3: Data Persistence Across Sessions
- **Description**: Verify data survives application restart
- **Steps**:
  1. Add recipes and create meal plan
  2. Quit application (`q`)
  3. Restart application
  4. Verify all data is intact
- **Expected**: All recipes, meal plan, and grocery list persist

### Scenario 4: Grocery List Check-Off Persistence
- **Description**: Verify check-off state persists after regeneration
- **Steps**:
  1. Generate grocery list
  2. Check off some items
  3. Quit and restart
  4. Verify check-off state persists
  5. Regenerate grocery list
  6. Verify check-off state resets (per BR-10)
- **Expected**: Check-off persists across sessions, resets on regeneration

## Manual Testing (TUI)
Since this is a TUI application, integration testing is primarily manual:
```bash
cargo run
```
Follow the scenarios above using keyboard navigation.
