use crate::domain::users::models::user::User;
use std::error::Error;

pub trait UserDao {
    fn signup(
        &self,
        id: String,
        email: String,
        password_hash: String,
    ) -> Result<User, Box<dyn Error>>;
    fn signin(&self, email: String, password_hash: String) -> Result<User, Box<dyn Error>>;
}
