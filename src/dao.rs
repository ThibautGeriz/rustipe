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
        let connexion = self.pool.get()?;

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

    fn get_recipe(&self, id: String) -> Result<domain::Recipe, Box<dyn Error>> {
        use crate::schema::ingredients::dsl::step_number as ingredient_step_number;
        use crate::schema::instructions::dsl::step_number as instructions_step_number;
        use crate::schema::recipes::dsl::{id as recipe_id, recipes};
        let connexion = self.pool.get()?;

        let recipes_results = recipes
            .filter(recipe_id.eq(id))
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
        let results: Vec<domain::Recipe> = data
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
            .collect();
        Ok(results.into_iter().last().unwrap())
    }

    fn delete_recipe(&self, id: String) -> Result<(), Box<dyn Error>> {
        use crate::schema::ingredients::dsl::{ingredients, recipe_id as ingredients_recipe_id};
        use crate::schema::instructions::dsl::{instructions, recipe_id as instructions_recipe_id};
        use crate::schema::recipes::dsl::{id as recipe_id, recipes};

        let connexion = self.pool.get()?;
        diesel::delete(ingredients.filter(ingredients_recipe_id.eq(id.clone())))
            .execute(&connexion)?;
        diesel::delete(instructions.filter(instructions_recipe_id.eq(id.clone())))
            .execute(&connexion)?;
        diesel::delete(recipes.filter(recipe_id.eq(id))).execute(&connexion)?;
        Ok(())
    }

    fn add_recipe<'a>(
        &self,
        id: &'a str,
        user_id: &'a str,
        title: &'a str,
        instructions: Vec<&'a str>,
        ingredients: Vec<&'a str>,
    ) -> Result<domain::Recipe, Box<dyn Error>> {
        use crate::schema::{ingredients, instructions, recipes};
        let connexion = self.pool.get()?;

        let new_recipe = NewRecipe { id, title, user_id };

        let inserted_recipe: Recipe = diesel::insert_into(recipes::table)
            .values(&new_recipe)
            .get_result(&connexion)?;

        let new_instructions: Vec<NewInstruction> = instructions
            .iter()
            .enumerate()
            .map(|(i, instuction)| NewInstruction {
                recipe_id: id,
                step_number: i as i32 + 1,
                instruction: instuction,
            })
            .collect();

        let inserted_instructions: Vec<Instruction> = diesel::insert_into(instructions::table)
            .values(&new_instructions)
            .get_results(&connexion)?;

        let new_ingredients: Vec<NewIngredient> = ingredients
            .iter()
            .enumerate()
            .map(|(i, ingredient)| NewIngredient {
                recipe_id: id,
                step_number: i as i32 + 1,
                ingredient,
            })
            .collect();

        let inserted_ingredients: Vec<Ingredient> = diesel::insert_into(ingredients::table)
            .values(&new_ingredients)
            .get_results(&connexion)?;

        Ok(domain::Recipe::from(
            &inserted_recipe,
            inserted_instructions,
            inserted_ingredients,
        ))
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

impl domain::Recipe {
    fn from(recipe: &Recipe, instructions: Vec<Instruction>, ingredients: Vec<Ingredient>) -> Self {
        let id = Uuid::parse_str(recipe.id.as_str()).expect("Cannot parse UUID");
        domain::Recipe {
            id,
            // TODO: remove those 4 unecessary clone
            user_id: recipe.user_id.clone(),
            title: recipe.title.clone(),
            instructions: instructions.iter().map(|i| i.instruction.clone()).collect(),
            ingredients: ingredients.iter().map(|i| i.ingredient.clone()).collect(),
        }
    }
}
