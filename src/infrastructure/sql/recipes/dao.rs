use crate::diesel::prelude::*;
use crate::domain::recipes::errors::RecipeError;
use crate::domain::recipes::models::recipe::Recipe as DomainRecipe;
use crate::domain::recipes::ports::dao::{NewRecipe as DomainNewRecipe, RecipeDao};
use crate::infrastructure::sql::models::*;

use itertools::izip;
use std::error::Error;
use uuid::Uuid;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub struct DieselRecipeDao {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl RecipeDao for DieselRecipeDao {
    fn get_my_recipes(
        &self,
        user_id: &str,
        query: Option<String>,
    ) -> Result<Vec<DomainRecipe>, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::ingredients::dsl::step_number as ingredient_step_number;
        use crate::infrastructure::sql::schema::instructions::dsl::step_number as instructions_step_number;
        use crate::infrastructure::sql::schema::recipes::dsl::{
            recipes, title, user_id as recipes_user_id,
        };
        let q = match query {
            Some(mut q) => {
                q.push('%');
                q
            }
            None => String::from("%"),
        };

        let recipes_results = recipes
            .filter(recipes_user_id.eq(user_id))
            .filter(title.ilike(&q))
            .load::<Recipe>(&self.connection)?;

        let instructions_results = Instruction::belonging_to(&recipes_results)
            .order_by(instructions_step_number.asc())
            .load::<Instruction>(&self.connection)?
            .grouped_by(&recipes_results);

        let ingredients_results = Ingredient::belonging_to(&recipes_results)
            .order_by(ingredient_step_number.asc())
            .load::<Ingredient>(&self.connection)?
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

        let recipes_results = recipes
            .filter(recipe_id.eq(id))
            .load::<Recipe>(&self.connection)?;

        let instructions_results = Instruction::belonging_to(&recipes_results)
            .order_by(instructions_step_number.asc())
            .load::<Instruction>(&self.connection)?
            .grouped_by(&recipes_results);

        let ingredients_results = Ingredient::belonging_to(&recipes_results)
            .order_by(ingredient_step_number.asc())
            .load::<Ingredient>(&self.connection)?
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
        let r = results
            .into_iter()
            .last()
            .ok_or(RecipeError::RecipeNotFound)?;
        Ok(r)
    }

    fn delete_recipe(&self, id: String) -> Result<(), Box<dyn Error>> {
        use crate::infrastructure::sql::schema::ingredients::dsl::{
            ingredients, recipe_id as ingredients_recipe_id,
        };
        use crate::infrastructure::sql::schema::instructions::dsl::{
            instructions, recipe_id as instructions_recipe_id,
        };
        use crate::infrastructure::sql::schema::recipes::dsl::{id as recipe_id, recipes};

        diesel::delete(ingredients.filter(ingredients_recipe_id.eq(id.clone())))
            .execute(&self.connection)?;
        diesel::delete(instructions.filter(instructions_recipe_id.eq(id.clone())))
            .execute(&self.connection)?;
        diesel::delete(recipes.filter(recipe_id.eq(id))).execute(&self.connection)?;
        Ok(())
    }

    fn update_recipe(&self, recipe: DomainRecipe) -> Result<DomainRecipe, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::ingredients::dsl::{
            ingredients, recipe_id as ingredients_recipe_id,
        };
        use crate::infrastructure::sql::schema::instructions::dsl::{
            instructions, recipe_id as instructions_recipe_id,
        };
        use crate::infrastructure::sql::schema::recipes::dsl::{
            category, cook_time_in_minute, cuisine, description, id as recipe_id, image_url,
            imported_from, prep_time_in_minute, recipe_yield, recipes, title,
        };
        let id: String = recipe.id.to_hyphenated().to_string();
        let inserted_recipe = diesel::update(recipes.filter(recipe_id.eq(&id)))
            .set((
                title.eq(recipe.title),
                description.eq(recipe.description),
                cook_time_in_minute.eq(recipe.cook_time_in_minute),
                prep_time_in_minute.eq(recipe.prep_time_in_minute),
                image_url.eq(recipe.image_url),
                recipe_yield.eq(recipe.recipe_yield),
                category.eq(recipe.category),
                cuisine.eq(recipe.cuisine),
                imported_from.eq(recipe.imported_from),
            ))
            .get_result(&self.connection)?;

        diesel::delete(ingredients.filter(ingredients_recipe_id.eq(&id)))
            .execute(&self.connection)?;
        diesel::delete(instructions.filter(instructions_recipe_id.eq(&id)))
            .execute(&self.connection)?;

        let inserted_instructions = self.insert_instructions(
            recipe.id.to_hyphenated().to_string().as_str(),
            &recipe
                .instructions
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        )?;

        let inserted_ingredients = self.insert_ingredients(
            recipe.id.to_hyphenated().to_string().as_str(),
            &recipe
                .ingredients
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        )?;
        Ok(DomainRecipe::from(
            &inserted_recipe,
            inserted_instructions,
            inserted_ingredients,
        ))
    }
    fn add_recipe(&self, new_recipe: DomainNewRecipe) -> Result<DomainRecipe, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::recipes;
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

        let inserted_recipe: Result<Recipe, diesel::result::Error> =
            diesel::insert_into(recipes::table)
                .values(&new_recipe_sql)
                .get_result(&self.connection);

        let inserted_recipe = inserted_recipe?;

        let inserted_instructions =
            self.insert_instructions(&new_recipe.id, &new_recipe.instructions)?;

        let inserted_ingredients =
            self.insert_ingredients(&new_recipe.id, &new_recipe.ingredients)?;

        Ok(DomainRecipe::from(
            &inserted_recipe,
            inserted_instructions,
            inserted_ingredients,
        ))
    }
}

impl DieselRecipeDao {
    pub fn new(connection: PooledConnection<ConnectionManager<PgConnection>>) -> DieselRecipeDao {
        DieselRecipeDao { connection }
    }

    fn insert_instructions<'a>(
        &self,
        recipe_id: &'a str,
        instructions_to_insert: &[&'a str],
    ) -> Result<Vec<Instruction>, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::instructions;

        let new_instructions: Vec<NewInstruction> = instructions_to_insert
            .iter()
            .enumerate()
            .map(|(i, instuction)| NewInstruction {
                recipe_id,
                step_number: i as i32 + 1,
                instruction: instuction,
            })
            .collect();

        let inserted_instructions: Vec<Instruction> = diesel::insert_into(instructions::table)
            .values(&new_instructions)
            .get_results(&self.connection)?;
        Ok(inserted_instructions)
    }

    fn insert_ingredients<'a>(
        &self,
        recipe_id: &'a str,
        ingredients_to_insert: &[&'a str],
    ) -> Result<Vec<Ingredient>, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::ingredients;
        let new_ingredients: Vec<NewIngredient> = ingredients_to_insert
            .iter()
            .enumerate()
            .map(|(i, ingredient)| NewIngredient {
                recipe_id,
                step_number: i as i32 + 1,
                ingredient,
            })
            .collect();

        let inserted_ingredients: Vec<Ingredient> = diesel::insert_into(ingredients::table)
            .values(&new_ingredients)
            .get_results(&self.connection)?;
        Ok(inserted_ingredients)
    }
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
