use crate::domain::recipes::errors::RecipeError;
use crate::domain::recipes::models::recipe::Recipe;
use crate::domain::recipes::ports::dao::{NewRecipe, RecipeDao};
use crate::domain::recipes::ports::parser::Parser;
use std::error::Error;
use std::marker::Send;
use std::marker::Sync;

pub struct RecipeInteractor {
    pub dao: Box<dyn RecipeDao>,
    pub parser: Box<dyn Parser + Send + Sync>,
}

impl RecipeInteractor {
    pub fn import_from(&self, url: String, user_id: String) -> Result<Recipe, Box<dyn Error>> {
        let new_recipe = &self.parser.parse_recipe(url, user_id)?;
        self.dao.add_recipe(NewRecipe {
            id: new_recipe.id.to_hyphenated().to_string().as_str(),
            user_id: &new_recipe.user_id.as_str(),
            title: &new_recipe.title.as_str(),
            description: new_recipe.description.as_deref(),
            recipe_yield: new_recipe.recipe_yield.as_deref(),
            category: new_recipe.category.as_deref(),
            cuisine: new_recipe.cuisine.as_deref(),
            prep_time_in_minute: (&new_recipe.prep_time_in_minute).as_ref(),
            cook_time_in_minute: (&new_recipe.cook_time_in_minute).as_ref(),
            instructions: new_recipe.instructions.iter().map(|s| s.as_str()).collect(),
            ingredients: new_recipe.ingredients.iter().map(|s| s.as_str()).collect(),
            imported_from: new_recipe.imported_from.as_deref(),
            image_url: new_recipe.image_url.as_deref(),
        })
    }

    pub fn add_recipe(&self, new_recipe: Recipe) -> Result<Recipe, Box<dyn Error>> {
        self.dao.add_recipe(NewRecipe {
            id: new_recipe.id.to_hyphenated().to_string().as_str(),
            user_id: &new_recipe.user_id.as_str(),
            title: &new_recipe.title.as_str(),
            description: new_recipe.description.as_deref(),
            recipe_yield: new_recipe.recipe_yield.as_deref(),
            category: new_recipe.category.as_deref(),
            cuisine: new_recipe.cuisine.as_deref(),
            prep_time_in_minute: (&new_recipe.prep_time_in_minute).as_ref(),
            cook_time_in_minute: (&new_recipe.cook_time_in_minute).as_ref(),
            instructions: new_recipe.instructions.iter().map(|s| s.as_str()).collect(),
            ingredients: new_recipe.ingredients.iter().map(|s| s.as_str()).collect(),
            imported_from: new_recipe.imported_from.as_deref(),
            image_url: new_recipe.image_url.as_deref(),
        })
    }

    pub fn update_recipe(&self, new_recipe: Recipe) -> Result<Recipe, Box<dyn Error>> {
        let recipe = self
            .dao
            .get_recipe(new_recipe.id.to_hyphenated().to_string())?;
        if recipe.user_id != new_recipe.user_id {
            return Err(Box::new(RecipeError::RecipeDoNotbelongToUser));
        }
        self.dao.update_recipe(new_recipe)
    }

    pub fn delete_recipe(&self, id: String, user_id: String) -> Result<(), Box<dyn Error>> {
        let recipe = self.dao.get_recipe(id.clone())?;
        if recipe.user_id != user_id {
            return Err(Box::new(RecipeError::RecipeDoNotbelongToUser));
        }
        self.dao.delete_recipe(id)
    }

    pub fn get_recipe(&self, id: String) -> Result<Recipe, Box<dyn Error>> {
        self.dao.get_recipe(id)
    }

    pub fn get_my_recipes(
        &self,
        user_id: String,
        query: Option<String>,
    ) -> Result<Vec<Recipe>, Box<dyn Error>> {
        self.dao.get_my_recipes(user_id, query)
    }
}
