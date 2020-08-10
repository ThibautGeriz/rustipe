use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
}
