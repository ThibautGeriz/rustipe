use crate::dao::DieselRecipeDao;
use crate::domain::{NewRecipe, Recipe, RecipeDao};
use juniper::FieldResult;
use uuid::Uuid;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A Recipe for a delicious dish")]
struct RecipeGraphQL {
    id: String,
    title: String,
    description: Option<String>,
    image_url: Option<String>,
    recipe_yield: Option<String>,
    category: Option<String>,
    cuisine: Option<String>,
    cook_time_in_minute: Option<i32>,
    prep_time_in_minute: Option<i32>,
    instructions: Vec<String>,
    ingredients: Vec<String>,
}

impl RecipeGraphQL {
    fn from(recipe: &Recipe) -> RecipeGraphQL {
        RecipeGraphQL {
            id: recipe.id.to_hyphenated().to_string(),
            title: recipe.title.clone(),
            description: recipe.description.clone(),
            image_url: recipe.image_url.clone(),
            recipe_yield: recipe.recipe_yield.clone(),
            category: recipe.category.clone(),
            cuisine: recipe.cuisine.clone(),
            cook_time_in_minute: recipe.cook_time_in_minute,
            prep_time_in_minute: recipe.prep_time_in_minute,
            instructions: recipe.instructions.clone(),
            ingredients: recipe.ingredients.clone(),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Recipe for a delicious dish")]
struct NewRecipeGraphQL {
    title: String,
    user_id: String,
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

    pub fn get_recipe(context: &Context, id: String) -> FieldResult<RecipeGraphQL> {
        let recipe = (&context.dao).get_recipe(id)?;
        Ok(RecipeGraphQL::from(&recipe))
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn createRecipe(context: &Context, new_recipe: NewRecipeGraphQL) -> FieldResult<RecipeGraphQL> {
        let recipe = (&context.dao).add_recipe(NewRecipe {
            id: Uuid::new_v4().to_string().as_str(),
            user_id: new_recipe.user_id.as_str(),
            title: new_recipe.title.as_str(),
            description: new_recipe.description.as_deref(),
            recipe_yield: new_recipe.recipe_yield.as_deref(),
            category: new_recipe.category.as_deref(),
            cuisine: new_recipe.cuisine.as_deref(),
            prep_time_in_minute: (&new_recipe.prep_time_in_minute).as_ref(),
            cook_time_in_minute: (&new_recipe.cook_time_in_minute).as_ref(),
            instructions: new_recipe.instructions.iter().map(|s| s.as_str()).collect(),
            ingredients: new_recipe.ingredients.iter().map(|s| s.as_str()).collect(),
            imported_from: new_recipe.imported_from.as_deref(),
        })?;
        Ok(RecipeGraphQL::from(&recipe))
    }

    fn deleteRecipe(context: &Context, id: String) -> FieldResult<String> {
        let recipe = (&context.dao).delete_recipe(id.clone())?;
        Ok(id)
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
