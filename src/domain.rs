use std::error::Error;
use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub struct Recipe {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
    pub cook_time_in_minute: Option<i32>,
    pub prep_time_in_minute: Option<i32>,
    pub image_url: Option<String>,
    pub recipe_yield: Option<String>,
    pub category: Option<String>,
    pub cuisine: Option<String>,
    pub instructions: Vec<String>,
    pub ingredients: Vec<String>,
    pub imported_from: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

trait UserDao {
    fn signup(email: String, password: String);
    fn signin(email: String, password: String);
}

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
