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
    fn add_recipe(
        &self,
        user_id: String,
        title: String,
        instructions: Vec<String>,
        ingredients: Vec<String>,
    ) -> Result<(), Box<dyn Error>>;
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
