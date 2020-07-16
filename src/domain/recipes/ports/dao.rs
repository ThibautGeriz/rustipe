use crate::domain::recipes::models::recipe::Recipe;
use std::error::Error;

#[derive(PartialEq, Debug)]
pub struct NewRecipe<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub recipe_yield: Option<&'a str>,
    pub category: Option<&'a str>,
    pub cuisine: Option<&'a str>,
    pub prep_time_in_minute: Option<&'a i32>,
    pub cook_time_in_minute: Option<&'a i32>,
    pub instructions: Vec<&'a str>,
    pub ingredients: Vec<&'a str>,
    pub imported_from: Option<&'a str>,
}

pub trait RecipeDao {
    fn get_my_recipes(&self, user_id: String) -> Result<Vec<Recipe>, Box<dyn Error>>;
    fn get_recipe(&self, id: String) -> Result<Recipe, Box<dyn Error>>;
    fn delete_recipe(&self, id: String) -> Result<(), Box<dyn Error>>;
    fn add_recipe(&self, new_recipe: NewRecipe) -> Result<Recipe, Box<dyn Error>>;
}
