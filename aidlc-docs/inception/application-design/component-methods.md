# Component Methods

> Note: Detailed business rules defined in Functional Design (CONSTRUCTION phase).

## RecipeStore

| Method | Signature | Purpose |
|---|---|---|
| add | `fn add(&mut self, recipe: Recipe) -> Result<Uuid>` | Add recipe, assign UUID, persist |
| list | `fn list(&self) -> Vec<&Recipe>` | Return all recipes |
| get | `fn get(&self, id: Uuid) -> Option<&Recipe>` | Get recipe by UUID |
| delete | `fn delete(&mut self, id: Uuid) -> Result<()>` | Remove recipe, persist |

## MealPlanner

| Method | Signature | Purpose |
|---|---|---|
| set_meal | `fn set_meal(&mut self, day: Day, slot: MealSlot, recipe_id: Uuid) -> Result<()>` | Assign recipe to day+slot |
| clear_meal | `fn clear_meal(&mut self, day: Day, slot: MealSlot) -> Result<()>` | Remove assignment |
| get_plan | `fn get_plan(&self) -> &MealPlan` | Return current week plan |
| get_meal | `fn get_meal(&self, day: Day, slot: MealSlot) -> Option<Uuid>` | Get recipe ID at day+slot |

## GroceryListGenerator

| Method | Signature | Purpose |
|---|---|---|
| generate | `fn generate(&self, plan: &MealPlan, recipes: &RecipeStore, household_size: u32) -> GroceryList` | Build consolidated grocery list |
| check_off | `fn check_off(list: &mut GroceryList, item_index: usize) -> Result<()>` | Mark item purchased |
| uncheck | `fn uncheck(list: &mut GroceryList, item_index: usize) -> Result<()>` | Unmark item |
| remaining | `fn remaining(list: &GroceryList) -> Vec<&GroceryItem>` | Return unchecked items |

## ScalingUtil

| Method | Signature | Purpose |
|---|---|---|
| scale_quantity | `fn scale_quantity(quantity: f64, recipe_servings: u32, household_size: u32) -> f64` | Proportional scaling |
| scale_ingredients | `fn scale_ingredients(ingredients: &[Ingredient], recipe_servings: u32, household_size: u32) -> Vec<Ingredient>` | Scale all ingredients |

## Storage

| Method | Signature | Purpose |
|---|---|---|
| load | `fn load<T: DeserializeOwned>(path: &Path) -> Result<T>` | Read and deserialize JSON |
| save | `fn save<T: Serialize>(path: &Path, data: &T) -> Result<()>` | Serialize and write JSON |
| data_dir | `fn data_dir() -> PathBuf` | Return data directory path |
