use crate::schema::{ingredients, instructions, recipes, users};

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub email: String,
    pub user_id: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "recipes"]
#[belongs_to(User)]
pub struct Recipe {
    pub id: String,
    pub user_id: String,
    pub title: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Recipe)]
#[primary_key(recipe_id, step_number)]
#[table_name = "ingredients"]
pub struct Ingredient {
    pub step_number: i32,
    pub recipe_id: String,
    pub ingredient: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Recipe)]
#[primary_key(recipe_id, step_number)]
#[table_name = "instructions"]
pub struct Instruction {
    pub step_number: i32,
    pub recipe_id: String,
    pub instruction: String,
}
