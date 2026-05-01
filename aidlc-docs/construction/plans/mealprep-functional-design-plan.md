# Functional Design Plan — MealPrep (Single Unit)

## Plan Steps

- [x] Define domain entities and relationships
- [x] Define business rules and validation logic
- [x] Define business logic model (algorithms and data flow)
- [x] Identify PBT testable properties (PBT-01)
- [x] Generate functional design artifacts

## Design Questions

Please answer the following questions to guide the functional design.

### Ingredient Modeling

## Question 1
How should ingredient quantities handle different units (e.g., "2 cups flour" + "100g flour")?

A) Ingredients are matched by name only — same name = same ingredient regardless of unit (user ensures consistency)
B) Ingredients are matched by name + unit — "flour (cups)" and "flour (g)" are treated as separate items
C) Implement unit conversion (cups ↔ grams ↔ ml, etc.) to merge across units
D) Other (please describe after [Answer]: tag below)

[Answer]: C

### Meal Slot Configuration

## Question 2
Should meal slots be fixed (breakfast, lunch, dinner) or user-configurable?

A) Fixed — exactly 3 slots: breakfast, lunch, dinner
B) Configurable — user can define custom slots (e.g., snack, second breakfast)
C) Other (please describe after [Answer]: tag below)

[Answer]: C. Set them configurable, but the default is to plan for the three slots: breakfast, lunch, and dinner.

### Grocery List Lifecycle

## Question 3
When should the grocery list be regenerated vs preserved?

A) Auto-regenerate every time the user views it (always reflects current plan); check-off state resets
B) Generate on explicit command only; check-off state persists until user regenerates
C) Auto-regenerate but preserve check-off state for items that still appear in the new list
D) Other (please describe after [Answer]: tag below)

[Answer]: B

### Recipe Deletion Safety

## Question 4
What happens when a user deletes a recipe that is currently assigned in the meal plan?

A) Block deletion — refuse to delete until recipe is removed from all meal slots
B) Cascade — delete recipe and remove it from all meal plan slots automatically
C) Warn and confirm — show affected meal slots, ask user to confirm
D) Other (please describe after [Answer]: tag below)

[Answer]: C

### Data Directory

## Question 5
Where should MealPrep store its JSON data files?

A) Current working directory (e.g., `./mealprep-data/`)
B) User's home config directory (e.g., `~/.config/mealprep/`)
C) XDG-compliant — use `dirs` crate for platform-appropriate data directory
D) Other (please describe after [Answer]: tag below)

[Answer]: C
