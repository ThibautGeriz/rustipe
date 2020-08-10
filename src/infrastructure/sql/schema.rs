table! {
    ingredients (recipe_id, step_number) {
        step_number -> Int4,
        recipe_id -> Varchar,
        ingredient -> Text,
    }
}

table! {
    instructions (recipe_id, step_number) {
        step_number -> Int4,
        recipe_id -> Varchar,
        instruction -> Text,
    }
}

table! {
    recipes (id) {
        id -> Varchar,
        user_id -> Varchar,
        title -> Varchar,
        cook_time_in_minute -> Nullable<Int4>,
        prep_time_in_minute -> Nullable<Int4>,
        description -> Nullable<Varchar>,
        image_url -> Nullable<Varchar>,
        recipe_yield -> Nullable<Varchar>,
        category -> Nullable<Varchar>,
        cuisine -> Nullable<Varchar>,
        imported_from -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(ingredients -> recipes (recipe_id));
joinable!(instructions -> recipes (recipe_id));
joinable!(recipes -> users (user_id));

allow_tables_to_appear_in_same_query!(ingredients, instructions, recipes, users,);
