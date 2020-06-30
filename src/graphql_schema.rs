use crate::dao::DieselRecipeDao;
use crate::domain::{Recipe, RecipeDao};
use juniper::FieldResult;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A Recipe for a delicious dish")]
struct RecipeGraphQL {
    id: String,
    title: String,
    instructions: Vec<String>,
    ingredients: Vec<String>,
}

impl RecipeGraphQL {
    fn from(recipe: &Recipe) -> RecipeGraphQL {
        RecipeGraphQL {
            id: recipe.id.to_hyphenated().to_string(),
            title: recipe.title.clone(),
            instructions: recipe.instructions.clone(),
            ingredients: recipe.ingredients.clone(),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Recipe for a delicious dish")]
struct NewRecipeGraphQL {
    title: String,
    instructions: Vec<String>,
    ingredients: Vec<String>,
}

pub struct Context {
    dao: DieselRecipeDao,
}

impl Context {
    pub fn new() -> Context {
        Context {
            dao: DieselRecipeDao::new(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    pub fn apiVersion() -> &str {
        "1.0"
    }
    pub fn get_my_recipes(context: &Context, user_id: String) -> FieldResult<Vec<RecipeGraphQL>> {
        let recipes = (&context.dao).get_my_recipes(user_id)?;
        Ok(recipes.iter().map(|r| RecipeGraphQL::from(r)).collect())
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    // fn createRecipe(context: &Context, new_recipe: NewRecipeGraphQL) -> FieldResult<Human> {
    //   let db = executor.context().pool.get_connection()?;
    //   let human: Human = db.insert_human(&new_human)?;
    //   Ok(human)
    // }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
