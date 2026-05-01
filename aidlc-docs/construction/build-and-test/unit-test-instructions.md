# Unit Test Execution

## Run All Tests
```bash
cd mealprep
cargo test
```

## Expected Results
- **Total Tests**: 10
- **Expected**: 10 pass, 0 failures

## Test Breakdown

| Test File | Tests | Properties Covered |
|---|---|---|
| `tests/models_pbt.rs` | 4 | TP-07 (Recipe), TP-08 (MealPlan), TP-09 (GroceryList), TP-10 (Config) serde round-trips |
| `tests/scaling_pbt.rs` | 3 | TP-01 (scaling round-trip), TP-03 (proportionality), TP-06 (identity) |
| `tests/units_pbt.rs` | 1 | TP-02 (unit conversion round-trip) |
| `tests/grocery_pbt.rs` | 2 | TP-04 (quantity preservation), TP-05 (check-off toggle + item count) |

## PBT Configuration
- **Framework**: proptest 1.5.0
- **Default cases**: 256 per property (grocery consolidation: 100)
- **Shrinking**: Enabled (default proptest behavior)
- **Seed logging**: Enabled — on failure, proptest prints seed and saves to `.proptest-regressions`

## Run Specific Test File
```bash
cargo test --test models_pbt
cargo test --test scaling_pbt
cargo test --test units_pbt
cargo test --test grocery_pbt
```

## Run with Verbose Output
```bash
cargo test -- --nocapture
```

## Reproduce a Failed PBT Run
If a test fails, proptest saves the failing case. Re-run to reproduce:
```bash
cargo test --test <test_file>
```
The `.proptest-regressions` file ensures the same failing input is retested.
