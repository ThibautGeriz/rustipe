use std::error::Error;
use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub struct Recipe {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
    pub instructions: Vec<String>,
    pub ingredients: Vec<String>,
}

pub trait RecipeDao {
    fn get_my_recipes(&self, user_id: String) -> Result<Vec<Recipe>, Box<dyn Error>>;
    fn get_recipe(&self, id: String) -> Result<Recipe, Box<dyn Error>>;
    fn delete_recipe(&self, id: String) -> Result<(), Box<dyn Error>>;
    fn add_recipe<'a>(
        &self,
        id: &'a str,
        user_id: &'a str,
        title: &'a str,
        instructions: Vec<&'a str>,
        ingredients: Vec<&'a str>,
    ) -> Result<Recipe, Box<dyn Error>>;
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
