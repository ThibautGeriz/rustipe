use crate::infrastructure::sql::schema::{ingredients, instructions, recipes, users};

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
    pub cook_time_in_minute: Option<i32>,
    pub prep_time_in_minute: Option<i32>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub recipe_yield: Option<String>,
    pub category: Option<String>,
    pub cuisine: Option<String>,
    pub imported_from: Option<String>,
}

#[derive(Insertable)]
#[table_name = "recipes"]
pub struct NewRecipe<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub user_id: &'a str,
    pub description: Option<&'a str>,
    pub cook_time_in_minute: Option<&'a i32>,
    pub prep_time_in_minute: Option<&'a i32>,
    pub image_url: Option<&'a str>,
    pub recipe_yield: Option<&'a str>,
    pub category: Option<&'a str>,
    pub cuisine: Option<&'a str>,
    pub imported_from: Option<&'a str>,
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

#[derive(Insertable)]
#[table_name = "ingredients"]
pub struct NewIngredient<'a> {
    pub step_number: i32,
    pub recipe_id: &'a str,
    pub ingredient: &'a str,
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

#[derive(Insertable)]
#[table_name = "instructions"]
pub struct NewInstruction<'a> {
    pub step_number: i32,
    pub recipe_id: &'a str,
    pub instruction: &'a str,
}
