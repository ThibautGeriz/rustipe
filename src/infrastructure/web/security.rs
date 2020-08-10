use crate::infrastructure::web::jwt::decode_header;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use uuid::Uuid;

pub struct LoggedUser(pub Option<Uuid>);

impl<'a, 'r> FromRequest<'a, 'r> for LoggedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let authorization_headers: Vec<_> = request.headers().get("Authorization").collect();
        if authorization_headers.len() != 1 {
            return Outcome::Success(LoggedUser(Option::None));
        }
        let authorization_header = authorization_headers.first().unwrap();
        if !authorization_header.starts_with("Bearer ") {
            return Outcome::Success(LoggedUser(Option::None));
        }
        let authorization_header: Vec<&str> = authorization_header.split(' ').collect();
        let token = authorization_header.last().unwrap();
        match decode_header(token) {
            Result::Ok(user_id) => Outcome::Success(LoggedUser(Option::Some(user_id))),
            _ => Outcome::Success(LoggedUser(Option::None)),
        }
    }
}
