use crate::diesel::prelude::*;
use crate::diesel::result::DatabaseErrorKind;
use crate::diesel::result::Error as DieselError;
use crate::domain::users::errors::UserError;
use crate::domain::users::models::user::User as DomainUser;
use crate::domain::users::ports::dao::UserDao;
use crate::infrastructure::sql::models::*;

use std::error::Error;
use uuid::Uuid;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub struct DieselUserDao {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl UserDao for DieselUserDao {
    fn signup(
        &self,
        id: String,
        email: String,
        password_hash: String,
    ) -> Result<DomainUser, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::users;

        let new_user_sql = NewUser {
            id: &id,
            email: &email,
            password_hash: &password_hash,
        };

        let inserted_user: User = diesel::insert_into(users::table)
            .values(&new_user_sql)
            .get_result(&self.connection)
            .map_err(|err| match err {
                DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    UserError::UserAlreadyExists
                }
                _ => UserError::Unknown,
            })?;

        Ok(DomainUser {
            id: Uuid::parse_str(inserted_user.id.as_str()).expect("Cannot parse UUID"),
            email: inserted_user.email,
        })
    }
    fn signin(&self, email: String, password_hash: String) -> Result<DomainUser, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::users::dsl::{
            email as db_email, password_hash as db_password_hash, users,
        };

        let users_results = users
            .filter(db_email.eq(email))
            .filter(db_password_hash.eq(password_hash))
            .load::<User>(&self.connection)?;

        let user: &User = users_results.first().ok_or(UserError::BadCredentials)?;
        Ok(DomainUser {
            id: Uuid::parse_str(user.id.as_str()).expect("Cannot parse UUID"),
            email: user.email.clone(),
        })
    }

    fn get_user(&self, id: &str) -> Result<DomainUser, Box<dyn Error>> {
        use crate::infrastructure::sql::schema::users::dsl::{id as db_id, users};

        let users_results = users.filter(db_id.eq(id)).load::<User>(&self.connection)?;

        let user: &User = users_results.first().ok_or(UserError::UserNotFound)?;
        Ok(DomainUser {
            id: Uuid::parse_str(user.id.as_str()).expect("Cannot parse UUID"),
            email: user.email.clone(),
        })
    }
}

impl DieselUserDao {
    pub fn new(connection: PooledConnection<ConnectionManager<PgConnection>>) -> DieselUserDao {
        DieselUserDao { connection }
    }
}
