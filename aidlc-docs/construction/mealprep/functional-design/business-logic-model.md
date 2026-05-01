# Business Logic Model — MealPrep

## Core Algorithms

### ALG-01: Ingredient Scaling

```
fn scale_quantity(quantity: f64, recipe_servings: u32, household_size: u32) -> f64:
    return quantity * (household_size as f64 / recipe_servings as f64)
```

**Properties**: Pure function, no side effects. Proportional relationship.

### ALG-02: Unit Conversion

```
fn to_base_unit(quantity: f64, unit: Unit) -> (f64, UnitGroup):
    match unit:
        Grams       -> (quantity, Weight)
        Kilograms   -> (quantity * 1000.0, Weight)
        Ounces      -> (quantity * 28.3495, Weight)
        Pounds      -> (quantity * 453.592, Weight)
        Milliliters -> (quantity, Volume)
        Liters      -> (quantity * 1000.0, Volume)
        Teaspoons   -> (quantity * 4.929, Volume)
        Tablespoons -> (quantity * 14.787, Volume)
        Cups        -> (quantity * 236.588, Volume)
        Pieces      -> (quantity, Count)
        Custom(s)   -> (quantity, Custom(s))

fn from_base_unit(quantity: f64, group: UnitGroup) -> (f64, Unit):
    match group:
        Weight -> if quantity >= 1000.0: (quantity / 1000.0, Kilograms)
                  else: (quantity, Grams)
        Volume -> if quantity >= 1000.0: (quantity / 1000.0, Liters)
                  else if quantity >= 236.588: (quantity / 236.588, Cups)
                  else: (quantity, Milliliters)
        Count  -> (quantity, Pieces)
        Custom(s) -> (quantity, Custom(s))
```

### ALG-03: Grocery List Generation

```
fn generate(plan: MealPlan, recipes: RecipeStore, household_size: u32) -> GroceryList:
    1. merged = HashMap<(lowercase_name, UnitGroup), f64>
    2. for each assignment in plan.assignments:
        recipe = recipes.get(assignment.recipe_id)
        scaled = scale_ingredients(recipe.ingredients, recipe.default_servings, household_size)
        for each ingredient in scaled:
            (base_qty, group) = to_base_unit(ingredient.quantity, ingredient.unit)
            key = (ingredient.name.to_lowercase(), group)
            merged[key] += base_qty
    3. items = []
       for each (name, group), total_qty in merged:
           (display_qty, display_unit) = from_base_unit(total_qty, group)
           items.push(GroceryItem { name, quantity: display_qty, unit: display_unit, checked: false })
    4. items.sort_by(name)
    5. return GroceryList { items, generated_at: now() }
```

### ALG-04: Ingredient Merging (within consolidation)

Same-name ingredients in the same unit group are summed in base units. Different unit groups produce separate items.

Example:
- "flour" 2 cups + "flour" 100g → two items (volume group + weight group, no cross-group merge)
- "flour" 2 cups + "flour" 0.5 cups → one item: 2.5 cups (same group, merged)

## Testable Properties (PBT-01)

### TP-01: Scaling Round-Trip (PBT-02 — Round-Trip)
- `scale_quantity(scale_quantity(q, s, h), h, s) ≈ q` (within floating-point tolerance)
- Scaling up then scaling back down returns original quantity

### TP-02: Unit Conversion Round-Trip (PBT-02 — Round-Trip)
- `from_base_unit(to_base_unit(q, unit)) ≈ (q, unit)` for standard units within same group
- Converting to base and back preserves quantity (within tolerance)

### TP-03: Scaling Invariant — Proportionality (PBT-03 — Invariant)
- `scale_quantity(q, s, h) / q == h / s` for all valid inputs
- Scaling factor is constant regardless of quantity

### TP-04: Consolidation Invariant — Total Quantity Preserved (PBT-03 — Invariant)
- Total base-unit quantity of an ingredient across all meals == total base-unit quantity in grocery list
- Merging does not create or destroy quantity

### TP-05: Consolidation Invariant — Item Count (PBT-03 — Invariant)
- Number of unique (name, unit_group) pairs in input == number of items in output
- No items lost or duplicated during consolidation

### TP-06: Scaling Idempotence Check (PBT-04 — Idempotence)
- `scale_quantity(q, s, s) == q` — scaling with household_size == recipe_servings is identity

### TP-07: Recipe Serialization Round-Trip (PBT-02 — Round-Trip)
- `deserialize(serialize(recipe)) == recipe` for all valid Recipe instances
- serde JSON round-trip preserves all fields

### TP-08: MealPlan Serialization Round-Trip (PBT-02 — Round-Trip)
- `deserialize(serialize(mealplan)) == mealplan`

### TP-09: GroceryList Serialization Round-Trip (PBT-02 — Round-Trip)
- `deserialize(serialize(grocerylist)) == grocerylist`

### TP-10: Config Serialization Round-Trip (PBT-02 — Round-Trip)
- `deserialize(serialize(config)) == config`
