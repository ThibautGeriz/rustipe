use crate::schema::{ingredients, instructions, recipes};

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "recipes"]
pub struct Recipe {
    pub id: i32,
    pub title: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Recipe)]
#[table_name = "ingredients"]
pub struct Ingredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Recipe)]
#[table_name = "instructions"]
pub struct Instruction {
    pub id: i32,
    pub recipe_id: i32,
    pub step_number: i32,
    pub instruction: String,
}
