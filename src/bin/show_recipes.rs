extern crate diesel;
extern crate itertools;
extern crate recipes_backend;

use self::diesel::prelude::*;
use self::itertools::izip;
use self::models::*;
use self::recipes_backend::*;

fn main() {
    use recipes_backend::schema::recipes::dsl::*;

    let connection = establish_connection();
    let recipes_results = recipes
        .load::<Recipe>(&connection)
        .expect("Error loading recipes");

    let instructions_results = Instruction::belonging_to(&recipes_results)
        .load::<Instruction>(&connection)
        .expect("Error loading instructions")
        .grouped_by(&recipes_results);

    let ingredients_results = Ingredient::belonging_to(&recipes_results)
        .load::<Ingredient>(&connection)
        .expect("Error loading ingredients")
        .grouped_by(&recipes_results);

    let data = izip!(
        &recipes_results,
        &ingredients_results,
        &instructions_results
    );

    println!("Displaying {} recipes", recipes_results.len());
    for (r_recipe, r_ingredients, r_instructions) in data {
        println!("{}", r_recipe.title);
        println!("----------\nIngredients:");
        for i in r_ingredients {
            println!(" - {}", i.ingredient);
        }

        println!("\nSteps:");
        for s in r_instructions {
            println!(" {}. {}", s.step_number, s.instruction);
        }
    }
}
