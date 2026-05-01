# MealPrep - Requirements Document

## Intent Analysis

- **User Request**: Build MealPrep — a weekly meal planning CLI app with smart grocery lists
- **Request Type**: New Project (Greenfield)
- **Scope Estimate**: Multiple Components (recipe management, meal planning, grocery list generation)
- **Complexity Estimate**: Moderate
- **Depth Level**: Standard

## Project Overview

**Pitch**: Plan your week's meals in 5 minutes, get a consolidated grocery list.

**Hypothesis**: Busy professionals will use a meal planning app that saves 2+ hours per week.

**Success Metrics**:
- 40% weekly active after 4 weeks
- 5+ meals planned per week

**Timeline**: 2 hours for MVP

## Technical Decisions

| Decision | Choice | Source |
|---|---|---|
| Target Platform | TUI application (terminal UI) | Clarification Q1: C, Design Q4: D |
| Language/Runtime | Rust | Design clarification: language change |
| TUI Framework | Ratatui | Design Q4: D (clarified) |
| CLI Parsing | clap | Rust ecosystem standard |
| Data Storage | Local JSON files (separate, UUID-indexed) | Q2: A, Design Q2: C |
| Serialization | serde + serde_json | Rust ecosystem standard |
| Authentication | None (single-user MVP) | Q3: B |
| Serving Size | Global household size, auto-adjust all recipes | Q5: B |
| Grocery Consolidation | Merge identical ingredients, sum quantities | Q6: A |

## Functional Requirements

### FR-01: Recipe Management
- Users can add custom recipes via CLI commands
- Each recipe has: name, ingredients (with quantities and units), default serving size, and instructions
- Recipes are stored in a local JSON file
- Users can list, view, and delete recipes

### FR-02: Weekly Meal Calendar
- Users can assign recipes to days of the week (Mon–Sun) and meal slots (breakfast, lunch, dinner)
- Users can view the current week's meal plan
- Users can clear or replace a meal assignment
- Meal plan is stored in a local JSON file

### FR-03: Grocery List Generation
- Auto-generate a consolidated grocery list from the current week's meal plan
- Merge identical ingredients across all planned meals and sum their quantities
- Adjust ingredient quantities based on the global household size setting
- Display the grocery list in the CLI

### FR-04: Grocery List Check-Off
- Users can mark grocery items as purchased (checked off)
- Users can view remaining (unchecked) items
- Check-off state persists in the local JSON file

### FR-05: Serving Size Adjustment
- Users set a global household size (number of servings)
- All recipe ingredient quantities auto-adjust proportionally based on household size vs recipe default serving size
- Adjustment applies when generating the grocery list

## Non-Functional Requirements

### NFR-01: Performance
- CLI commands should respond in under 500ms for typical data sizes (up to 100 recipes, 21 meals/week)

### NFR-02: Data Persistence
- All data stored locally in JSON files in a user-accessible directory
- Data survives between CLI sessions
- No cloud storage or network calls required for MVP

### NFR-03: Usability
- Clear CLI help text for all commands
- Meaningful error messages for invalid input
- Consistent command structure (e.g., `mealprep recipe add`, `mealprep plan set`, `mealprep grocery list`)

### NFR-04: Security (Reduced Scope for MVP)
- **SECURITY-05 (Input Validation)**: Validate all CLI input parameters — type checking, length bounds, format validation
- **SECURITY-05 (Injection Prevention)**: Sanitize inputs before writing to JSON files; no eval or dynamic code execution on user input
- All other security rules deferred to post-MVP (no auth, no network, no cloud storage)

### NFR-05: Testability
- Property-based testing enforced (full PBT extension)
- PBT framework: proptest (Rust PBT with shrinking and seed reproducibility)
- Key PBT targets: ingredient quantity scaling (invariants), grocery list consolidation (invariants), recipe serialization/deserialization (round-trip)

## Out of Scope (MVP)
- User authentication / multi-user support
- Cloud deployment / API endpoints
- Nutritional information
- Recipe import from URLs
- Store aisle grouping for grocery lists
- Mobile or web UI

## Extension Compliance

| Extension | Status | Enforcement |
|---|---|---|
| Security Baseline | Enabled | Reduced — SECURITY-05 only (input validation + injection prevention) |
| Property-Based Testing | Enabled | Full — all PBT rules enforced |
