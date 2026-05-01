# Business Rules — MealPrep

## Validation Rules

### BR-01: Recipe Name
- Required, non-empty, max 200 characters
- Trimmed of leading/trailing whitespace

### BR-02: Ingredient Name
- Required, non-empty, max 100 characters
- Trimmed of leading/trailing whitespace
- Case-insensitive matching for grocery list consolidation (e.g., "Flour" = "flour")

### BR-03: Ingredient Quantity
- Must be > 0
- Must be finite (no NaN, no Infinity)

### BR-04: Serving Size
- Recipe default_servings: must be ≥ 1
- Config household_size: must be ≥ 1

### BR-05: Meal Slot Names
- Required, non-empty, max 50 characters
- Must be unique within the slot list (case-insensitive)
- Default slots: ["breakfast", "lunch", "dinner"]

### BR-06: Meal Assignment
- Day must be a valid Day enum variant
- Slot must exist in MealPlan.slots
- recipe_id must reference an existing Recipe
- Only one recipe per (day, slot) pair — assigning overwrites

## Business Logic Rules

### BR-07: Ingredient Scaling
- `scaled_quantity = quantity * (household_size / recipe_default_servings)`
- Scaling is applied per-ingredient when generating the grocery list
- Scaling factor must be > 0 (guaranteed by BR-04)

### BR-08: Unit Conversion
Ingredients with the same name but different units are converted to a common unit before merging.

**Conversion groups** (within each group, convert to the base unit for merging):

| Group | Base Unit | Conversions |
|---|---|---|
| Weight | Grams | 1 kg = 1000g, 1 oz = 28.3495g, 1 lb = 453.592g |
| Volume | Milliliters | 1 L = 1000ml, 1 tsp = 4.929ml, 1 tbsp = 14.787ml, 1 cup = 236.588ml |
| Count | Pieces | No conversion needed |

**Rules**:
- Same group → convert to base unit, sum, then convert back to the larger unit if result exceeds threshold (e.g., >1000g → kg)
- Different groups (e.g., weight + volume) → keep as separate grocery items (no cross-group conversion)
- Custom units → match by exact unit string only; no conversion

### BR-09: Grocery List Consolidation
1. Collect all ingredients from all assigned meals in the plan
2. Scale each ingredient by household_size / recipe_default_servings
3. Group by (lowercase name, unit group)
4. Within each group: convert to base unit, sum quantities
5. Convert back to a human-friendly unit if appropriate
6. Sort alphabetically by ingredient name

### BR-10: Grocery List Lifecycle
- Generated only on explicit user command
- Check-off state persists until user explicitly regenerates
- Regeneration replaces the entire list (check-off state resets)

### BR-11: Recipe Deletion
- When deleting a recipe that appears in the meal plan:
  1. Identify all (day, slot) pairs referencing this recipe
  2. Display affected assignments to user
  3. Require user confirmation before proceeding
  4. On confirm: delete recipe AND remove all its meal assignments
  5. On cancel: abort deletion, no changes

### BR-12: Input Sanitization (SECURITY-05)
- All string inputs trimmed of leading/trailing whitespace
- String lengths validated against maximums before persistence
- No dynamic code execution on user input
- JSON serialization via serde (safe by default)
