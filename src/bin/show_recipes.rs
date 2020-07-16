extern crate diesel;
extern crate itertools;
extern crate recipes_backend;

use self::recipes_backend::domain::recipes::ports::dao::RecipeDao;
use self::recipes_backend::infrastructure::sql::recipes::dao::DieselRecipeDao;

fn main() {
    let dao = DieselRecipeDao::new();
    let recipes_results = dao
        .get_my_recipes(String::from("b8427f3a-ac40-4b62-9fe2-688b3b014161"))
        .expect("Cannot get recipes");

    println!("Displaying {} recipes", recipes_results.len());
    for recipe in recipes_results {
        println!("{}", recipe.title);
        println!("----------\nIngredients:");
        for i in recipe.ingredients {
            println!(" - {}", i);
        }

        println!("\nSteps:");
        for (i, instruction) in recipe.instructions.iter().enumerate() {
            println!(" {}. {}", i + 1, instruction);
        }
    }
}
