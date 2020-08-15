use crate::domain::recipes::interactors::recipe::RecipeInteractor;
use crate::domain::recipes::models::recipe::Recipe;
use crate::domain::users::errors::UserError;
use crate::domain::users::interactors::user::UserInteractor;
use crate::infrastructure::parser::html::SelectParser;
use crate::infrastructure::sql::recipes::dao::DieselRecipeDao;
use crate::infrastructure::sql::users::dao::DieselUserDao;
use crate::infrastructure::web::jwt::generate_header;
use crate::infrastructure::web::security::LoggedUser;
use crate::infrastructure::web::server::DbCon;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::Outcome;

use juniper::FieldResult;
use rocket::request::{self, FromRequest, Request};
use uuid::Uuid;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A Recipe for a delicious dish")]
struct RecipeGraphQL {
    id: String,
    title: String,
    user_id: String,
    description: Option<String>,
    image_url: Option<String>,
    recipe_yield: Option<String>,
    category: Option<String>,
    cuisine: Option<String>,
    cook_time_in_minute: Option<i32>,
    prep_time_in_minute: Option<i32>,
    instructions: Vec<String>,
    ingredients: Vec<String>,
    imported_from: Option<String>,
}

impl RecipeGraphQL {
    fn from(recipe: &Recipe) -> RecipeGraphQL {
        RecipeGraphQL {
            id: recipe.id.to_hyphenated().to_string(),
            title: recipe.title.clone(),
            user_id: recipe.user_id.clone(),
            description: recipe.description.clone(),
            image_url: recipe.image_url.clone(),
            recipe_yield: recipe.recipe_yield.clone(),
            category: recipe.category.clone(),
            cuisine: recipe.cuisine.clone(),
            cook_time_in_minute: recipe.cook_time_in_minute,
            prep_time_in_minute: recipe.prep_time_in_minute,
            instructions: recipe.instructions.clone(),
            ingredients: recipe.ingredients.clone(),
            imported_from: recipe.imported_from.clone(),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Recipe for a delicious dish")]
struct NewRecipeGraphQL {
    title: String,
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
    recipe_interactor: RecipeInteractor,
    user_interactor: UserInteractor,
    user_id: Option<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Context, Self::Error> {
        let user: LoggedUser = request.guard::<LoggedUser>()?;
        let db_con: DbCon = request.guard::<DbCon>()?;
        // TODO: fix this, should be borrowed but lost against compiler lifetime.
        let db_con2: DbCon = request.guard::<DbCon>()?;

        Outcome::Success(Context::new(
            db_con.0,
            db_con2.0,
            user.0.map(|user_id| user_id.to_hyphenated().to_string()),
        ))
    }
}

impl Context {
    pub fn new(
        connection: PooledConnection<ConnectionManager<PgConnection>>,
        connection2: PooledConnection<ConnectionManager<PgConnection>>,
        user_id: Option<String>,
    ) -> Context {
        Context {
            recipe_interactor: RecipeInteractor {
                dao: Box::new(DieselRecipeDao::new(connection)),
                parser: Box::new(SelectParser::new()),
            },
            user_interactor: UserInteractor {
                dao: Box::new(DieselUserDao::new(connection2)),
            },
            user_id,
        }
    }

    pub fn get_user(&self) -> FieldResult<String> {
        if self.user_id.is_none() {
            return Err(juniper::FieldError::from(UserError::MustBeLogged));
        }
        Ok(self.user_id.clone().unwrap())
    }
}

impl<'a> juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    pub fn apiVersion() -> &str {
        "1.0"
    }
    pub fn get_my_recipes(context: &Context) -> FieldResult<Vec<RecipeGraphQL>> {
        let user_id = context.get_user()?;
        let recipes = (&context.recipe_interactor).get_my_recipes(user_id)?;
        Ok(recipes.iter().map(|r| RecipeGraphQL::from(r)).collect())
    }

    pub fn get_recipe(context: &Context, id: String) -> FieldResult<RecipeGraphQL> {
        let recipe = (&context.recipe_interactor).get_recipe(id)?;
        Ok(RecipeGraphQL::from(&recipe))
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn createRecipe(context: &Context, new_recipe: NewRecipeGraphQL) -> FieldResult<RecipeGraphQL> {
        let user_id = context.get_user()?;
        let recipe = (&context.recipe_interactor).add_recipe(Recipe {
            id: Uuid::new_v4(),
            user_id,
            title: new_recipe.title,
            description: new_recipe.description,
            recipe_yield: new_recipe.recipe_yield,
            category: new_recipe.category,
            cuisine: new_recipe.cuisine,
            prep_time_in_minute: new_recipe.prep_time_in_minute,
            cook_time_in_minute: new_recipe.cook_time_in_minute,
            instructions: new_recipe.instructions,
            ingredients: new_recipe.ingredients,
            imported_from: new_recipe.imported_from,
            image_url: new_recipe.image_url,
        })?;

        Ok(RecipeGraphQL::from(&recipe))
    }

    fn updateRecipe(
        context: &Context,
        id: String,
        new_recipe: NewRecipeGraphQL,
    ) -> FieldResult<RecipeGraphQL> {
        let user_id = context.get_user()?;
        let recipe = (&context.recipe_interactor).update_recipe(Recipe {
            id: Uuid::parse_str(id.as_str()).expect("Cannot parse UUID"),
            user_id,
            title: new_recipe.title,
            description: new_recipe.description,
            recipe_yield: new_recipe.recipe_yield,
            category: new_recipe.category,
            cuisine: new_recipe.cuisine,
            prep_time_in_minute: new_recipe.prep_time_in_minute,
            cook_time_in_minute: new_recipe.cook_time_in_minute,
            instructions: new_recipe.instructions,
            ingredients: new_recipe.ingredients,
            imported_from: new_recipe.imported_from,
            image_url: new_recipe.image_url,
        })?;

        Ok(RecipeGraphQL::from(&recipe))
    }

    fn deleteRecipe(context: &Context, id: String) -> FieldResult<String> {
        let user_id = context.get_user()?;
        (&context.recipe_interactor).delete_recipe(id.clone(), user_id)?;
        Ok(id)
    }

    fn importRecipe(context: &Context, url: String) -> FieldResult<RecipeGraphQL> {
        let user_id = context.get_user()?;
        let recipe = (&context.recipe_interactor).import_from(url, user_id)?;
        Ok(RecipeGraphQL::from(&recipe))
    }

    fn signup(context: &Context, email: String, password: String) -> FieldResult<String> {
        let id = Uuid::new_v4();
        let user = (&context.user_interactor).signup(id, email, password)?;
        let jwt_token = generate_header(user)?;
        Ok(jwt_token)
    }

    fn signin(context: &Context, email: String, password: String) -> FieldResult<String> {
        let user = (&context.user_interactor).signin(email, password)?;
        let jwt_token = generate_header(user)?;
        Ok(jwt_token)
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
