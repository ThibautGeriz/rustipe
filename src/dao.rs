use crate::diesel::prelude::*;
use crate::domain;
use crate::domain::RecipeDao;
use crate::models::*;

use itertools::izip;
use std::error::Error;
use uuid::Uuid;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub struct DieselRecipeDao {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl RecipeDao for DieselRecipeDao {
    fn get_my_recipes(&self, user_id: String) -> Result<Vec<domain::Recipe>, Box<dyn Error>> {
        use crate::schema::ingredients::dsl::step_number as ingredient_step_number;
        use crate::schema::instructions::dsl::step_number as instructions_step_number;
        use crate::schema::recipes::dsl::{recipes, user_id as recipes_user_id};
        let connexion = self.pool.get().expect("Could not connect to the DB");

        let recipes_results = recipes
            .filter(recipes_user_id.eq(user_id))
            .load::<Recipe>(&connexion)?;

        let instructions_results = Instruction::belonging_to(&recipes_results)
            .order_by(instructions_step_number.asc())
            .load::<Instruction>(&connexion)?
            .grouped_by(&recipes_results);

        let ingredients_results = Ingredient::belonging_to(&recipes_results)
            .order_by(ingredient_step_number.asc())
            .load::<Ingredient>(&connexion)?
            .grouped_by(&recipes_results);

        let data = izip!(
            &recipes_results,
            &ingredients_results,
            &instructions_results
        );
        Ok(data
            .map(|(recipe, ingredients, instructions)| {
                let id = Uuid::parse_str(recipe.id.as_str()).expect("Cannot parse UUID");
                domain::Recipe {
                    id,
                    // TODO: remove those 4 unecessary clone (maybe move that in the diesel models)
                    user_id: recipe.user_id.clone(),
                    title: recipe.title.clone(),
                    instructions: instructions.iter().map(|i| i.instruction.clone()).collect(),
                    ingredients: ingredients.iter().map(|i| i.ingredient.clone()).collect(),
                }
            })
            .collect())
    }

    fn add_recipe(
        &self,
        _user_id: String,
        _title: String,
        _instructions: Vec<String>,
        _ingredients: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl DieselRecipeDao {
    pub fn new() -> DieselRecipeDao {
        DieselRecipeDao {
            pool: create_pool(),
        }
    }
}

impl Default for DieselRecipeDao {
    fn default() -> Self {
        DieselRecipeDao::new()
    }
}

fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    Pool::builder().max_size(10).build(manager).unwrap()
}
