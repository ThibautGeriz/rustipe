use crate::diesel::prelude::*;
use crate::domain::recipes::models::recipe::Recipe as DomainRecipe;
use crate::domain::recipes::ports::dao::{NewRecipe as DomainNewRecipe, RecipeDao};
use crate::infrastructure::sql::models::*;

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
    fn get_my_recipes(&self, user_id: String) -> Result<Vec<DomainRecipe>, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::ingredients::dsl::step_number as ingredient_step_number;
        use crate::infrastructure::sql::schema::instructions::dsl::step_number as instructions_step_number;
        use crate::infrastructure::sql::schema::recipes::dsl::{
            recipes, user_id as recipes_user_id,
        };
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
                DomainRecipe {
                    id,
                    // TODO: remove those unecessary clone (maybe move that in the diesel models)
                    user_id: recipe.user_id.clone(),
                    title: recipe.title.clone(),
                    description: recipe.description.clone(),
                    image_url: recipe.image_url.clone(),
                    recipe_yield: recipe.recipe_yield.clone(),
                    category: recipe.category.clone(),
                    cuisine: recipe.cuisine.clone(),
                    imported_from: recipe.imported_from.clone(),
                    cook_time_in_minute: recipe.cook_time_in_minute,
                    prep_time_in_minute: recipe.prep_time_in_minute,
                    instructions: instructions.iter().map(|i| i.instruction.clone()).collect(),
                    ingredients: ingredients.iter().map(|i| i.ingredient.clone()).collect(),
                }
            })
            .collect())
    }

    fn get_recipe(&self, id: String) -> Result<DomainRecipe, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::ingredients::dsl::step_number as ingredient_step_number;
        use crate::infrastructure::sql::schema::instructions::dsl::step_number as instructions_step_number;
        use crate::infrastructure::sql::schema::recipes::dsl::{id as recipe_id, recipes};
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
        let results: Vec<DomainRecipe> = data
            .map(|(recipe, ingredients, instructions)| {
                let id = Uuid::parse_str(recipe.id.as_str()).expect("Cannot parse UUID");
                DomainRecipe {
                    id,
                    // TODO: remove those unecessary clone (maybe move that in the diesel models)
                    user_id: recipe.user_id.clone(),
                    title: recipe.title.clone(),
                    description: recipe.description.clone(),
                    image_url: recipe.image_url.clone(),
                    recipe_yield: recipe.recipe_yield.clone(),
                    category: recipe.category.clone(),
                    cuisine: recipe.cuisine.clone(),
                    imported_from: recipe.imported_from.clone(),
                    cook_time_in_minute: recipe.cook_time_in_minute,
                    prep_time_in_minute: recipe.prep_time_in_minute,
                    instructions: instructions.iter().map(|i| i.instruction.clone()).collect(),
                    ingredients: ingredients.iter().map(|i| i.ingredient.clone()).collect(),
                }
            })
            .collect();
        Ok(results.into_iter().last().unwrap())
    }

    fn delete_recipe(&self, id: String) -> Result<(), Box<dyn Error>> {
        use crate::infrastructure::sql::schema::ingredients::dsl::{
            ingredients, recipe_id as ingredients_recipe_id,
        };
        use crate::infrastructure::sql::schema::instructions::dsl::{
            instructions, recipe_id as instructions_recipe_id,
        };
        use crate::infrastructure::sql::schema::recipes::dsl::{id as recipe_id, recipes};

        let connexion = self.pool.get()?;
        diesel::delete(ingredients.filter(ingredients_recipe_id.eq(id.clone())))
            .execute(&connexion)?;
        diesel::delete(instructions.filter(instructions_recipe_id.eq(id.clone())))
            .execute(&connexion)?;
        diesel::delete(recipes.filter(recipe_id.eq(id))).execute(&connexion)?;
        Ok(())
    }

    fn add_recipe(&self, new_recipe: DomainNewRecipe) -> Result<DomainRecipe, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::{ingredients, instructions, recipes};
        let connexion = self.pool.get()?;

        let new_recipe_sql = NewRecipe {
            id: new_recipe.id,
            title: new_recipe.title,
            user_id: new_recipe.user_id,
            image_url: new_recipe.image_url,
            description: new_recipe.description,
            recipe_yield: new_recipe.recipe_yield,
            category: new_recipe.category,
            cuisine: new_recipe.cuisine,
            prep_time_in_minute: new_recipe.prep_time_in_minute,
            cook_time_in_minute: new_recipe.cook_time_in_minute,
            imported_from: new_recipe.imported_from,
        };

        let inserted_recipe: Recipe = diesel::insert_into(recipes::table)
            .values(&new_recipe_sql)
            .get_result(&connexion)?;

        let new_instructions: Vec<NewInstruction> = new_recipe
            .instructions
            .iter()
            .enumerate()
            .map(|(i, instuction)| NewInstruction {
                recipe_id: new_recipe.id,
                step_number: i as i32 + 1,
                instruction: instuction,
            })
            .collect();

        let inserted_instructions: Vec<Instruction> = diesel::insert_into(instructions::table)
            .values(&new_instructions)
            .get_results(&connexion)?;

        let new_ingredients: Vec<NewIngredient> = new_recipe
            .ingredients
            .iter()
            .enumerate()
            .map(|(i, ingredient)| NewIngredient {
                recipe_id: new_recipe.id,
                step_number: i as i32 + 1,
                ingredient,
            })
            .collect();

        let inserted_ingredients: Vec<Ingredient> = diesel::insert_into(ingredients::table)
            .values(&new_ingredients)
            .get_results(&connexion)?;

        Ok(DomainRecipe::from(
            &inserted_recipe,
            inserted_instructions,
            inserted_ingredients,
        ))
    }
}

impl DieselRecipeDao {
    pub fn new(database_url: String) -> DieselRecipeDao {
        DieselRecipeDao {
            pool: create_pool(database_url),
        }
    }
}

impl Default for DieselRecipeDao {
    fn default() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        DieselRecipeDao::new(database_url)
    }
}

fn create_pool(database_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder().max_size(10).build(manager).unwrap()
}

impl DomainRecipe {
    fn from(recipe: &Recipe, instructions: Vec<Instruction>, ingredients: Vec<Ingredient>) -> Self {
        let id = Uuid::parse_str(recipe.id.as_str()).expect("Cannot parse UUID");
        DomainRecipe {
            id,
            // TODO: remove those 4 unecessary clone
            user_id: recipe.user_id.clone(),
            title: recipe.title.clone(),
            description: recipe.description.clone(),
            image_url: recipe.image_url.clone(),
            recipe_yield: recipe.recipe_yield.clone(),
            category: recipe.category.clone(),
            cuisine: recipe.cuisine.clone(),
            imported_from: recipe.imported_from.clone(),
            cook_time_in_minute: recipe.cook_time_in_minute,
            prep_time_in_minute: recipe.prep_time_in_minute,
            instructions: instructions.iter().map(|i| i.instruction.clone()).collect(),
            ingredients: ingredients.iter().map(|i| i.ingredient.clone()).collect(),
        }
    }
}
