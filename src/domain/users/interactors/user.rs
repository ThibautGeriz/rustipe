use crate::domain::users::models::user::User;
use crate::domain::users::ports::dao::UserDao;
use ring::{digest, pbkdf2};
use std::error::Error;
use std::marker::Send;
use std::marker::Sync;
use std::num::NonZeroU32;
use uuid::Uuid;

static PASSWORD_SALT: &str = "qi7263tjmxx'[";
static DIGEST_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const HASH_LEN: usize = digest::SHA256_OUTPUT_LEN;
type PasswordHash = [u8; HASH_LEN];

pub struct UserInteractor {
    pub dao: Box<dyn UserDao + Send + Sync>,
}

impl UserInteractor {
    pub fn signup(
        &self,
        id: Uuid,
        email: String,
        password: String,
    ) -> Result<User, Box<dyn Error>> {
        self.dao.signup(
            id.to_hyphenated().to_string(),
            email,
            UserInteractor::hash_password(&password),
        )
    }
    pub fn signin(&self, email: String, password: String) -> Result<User, Box<dyn Error>> {
        self.dao
            .signin(email, UserInteractor::hash_password(&password))
    }

    fn hash_password(password: &str) -> String {
        let mut to_store: PasswordHash = [0u8; HASH_LEN];
        pbkdf2::derive(
            DIGEST_ALG,
            NonZeroU32::new(1000).unwrap(),
            PASSWORD_SALT.as_bytes(),
            password.as_bytes(),
            &mut to_store,
        );
        UserInteractor::hash_to_string(to_store)
    }

    fn hash_to_string(hash: PasswordHash) -> String {
        let mut res = String::from("");
        for i in hash.iter() {
            res.push(*i as char)
        }
        res
    }
}
