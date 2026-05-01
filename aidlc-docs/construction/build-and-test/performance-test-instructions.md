# Performance Test Instructions

## Performance Requirements (NFR-01)
- CLI/TUI commands respond in under 500ms for typical data sizes
- Typical data: up to 100 recipes, 21 meals/week

## Manual Performance Validation

### 1. Generate Test Data
Create a `recipes.json` with 100 recipes in the data directory:
```bash
# Use the app to add recipes, or manually create a JSON file
# with 100 recipes, each having 5-10 ingredients
```

### 2. Measure Grocery List Generation
```bash
time cargo run --release
# Navigate to grocery view, press 'g' to generate
# Observe response time — should be near-instant
```

### 3. Expected Results
- **Startup time**: < 200ms (loading JSON files)
- **Grocery list generation**: < 100ms for 100 recipes × 21 meals
- **Navigation**: Instant (< 16ms per frame at 60fps)

## Notes
- Rust + Ratatui is inherently fast — performance issues are unlikely at MVP scale
- JSON file I/O is the primary bottleneck; at 100 recipes this is negligible
- No network calls, no database queries — all local computation
