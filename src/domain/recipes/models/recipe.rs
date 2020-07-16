use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub struct Recipe {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
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
