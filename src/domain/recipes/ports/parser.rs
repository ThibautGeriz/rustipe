use crate::domain::recipes::models::recipe::Recipe;
use std::error::Error;

pub trait Parser {
    fn parse_recipe(&self, url: String, user_id: String) -> Result<Recipe, Box<dyn Error>>;
}
