# Application Design Plan — MealPrep CLI

## Design Plan Steps

- [x] Identify core components and responsibilities
- [x] Define component method signatures and interfaces
- [x] Design service layer and orchestration
- [x] Map component dependencies and data flow
- [x] Validate design completeness and consistency
- [x] Generate design artifacts

## Design Questions

Please answer the following questions to guide the application design.

### Component Organization

## Question 1
How should the CLI command structure be organized?

A) Flat commands (e.g., `mealprep add-recipe`, `mealprep set-meal`, `mealprep grocery`)
B) Nested subcommands (e.g., `mealprep recipe add`, `mealprep plan set`, `mealprep grocery list`)
C) Other (please describe after [Answer]: tag below)

[Answer]: B

### Data Layer

## Question 2
How should JSON data files be organized?

A) Single file — all data (recipes, meal plans, grocery lists) in one `mealprep-data.json`
B) Separate files — `recipes.json`, `mealplan.json`, `grocerylist.json`
C) Other (please describe after [Answer]: tag below)

[Answer]: C: separate files, with uuid indexing recipes, mealplan, and grocerylist so that they can be merged.

### Component Boundaries

## Question 3
Should ingredient quantity scaling (household size adjustment) be handled by:

A) The Recipe component — recipes expose a method to return scaled ingredients
B) The GroceryList component — scaling happens during grocery list generation
C) A shared utility — a standalone scaling function used by both
D) Other (please describe after [Answer]: tag below)

[Answer]: D. It should be a shared utility, use uuid to index and link with the json files of Question 2.

### CLI Framework

## Question 4
Which CLI framework approach do you prefer?

A) Commander.js — widely used, mature, subcommand support
B) Yargs — feature-rich, auto-generated help, TypeScript support
C) Minimal — no framework, use process.argv with a simple parser
D) Other (please describe after [Answer]: tag below)

[Answer]: D) Create a terminal use interface (TUI) `https://charm.land/` as the framework
