use crate::diesel::prelude::*;
use crate::domain::users::errors::UserError;
use crate::domain::users::models::user::User as DomainUser;
use crate::domain::users::ports::dao::UserDao;
use crate::infrastructure::sql::models::*;

use std::error::Error;
use uuid::Uuid;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub struct DieselUserDao {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserDao for DieselUserDao {
    fn signup(
        &self,
        id: String,
        email: String,
        password_hash: String,
    ) -> Result<DomainUser, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::users;
        let connexion = self.pool.get()?;

        let new_user_sql = NewUser {
            id: &id,
            email: &email,
            password_hash: &password_hash,
        };

        let inserted_user: User = diesel::insert_into(users::table)
            .values(&new_user_sql)
            .get_result(&connexion)?;

        Ok(DomainUser {
            id: Uuid::parse_str(inserted_user.id.as_str()).expect("Cannot parse UUID"),
            email: inserted_user.email,
        })
    }
    fn signin(&self, email: String, password_hash: String) -> Result<DomainUser, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::users::dsl::{
            email as db_email, password_hash as db_password_hash, users,
        };
        let connexion = self.pool.get()?;

        let users_results = users
            .filter(db_email.eq(email))
            .filter(db_password_hash.eq(password_hash))
            .load::<User>(&connexion)?;

        let user: &User = users_results.first().ok_or(UserError::BadCredentials)?;
        Ok(DomainUser {
            id: Uuid::parse_str(user.id.as_str()).expect("Cannot parse UUID"),
            email: user.email.clone(),
        })
    }
}

impl DieselUserDao {
    pub fn new(database_url: String) -> DieselUserDao {
        DieselUserDao {
            pool: create_pool(database_url),
        }
    }
}

impl Default for DieselUserDao {
    fn default() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        DieselUserDao::new(database_url)
    }
}

fn create_pool(database_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder().max_size(10).build(manager).unwrap()
}
