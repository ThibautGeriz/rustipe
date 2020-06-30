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
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(ingredients -> recipes (recipe_id));
joinable!(instructions -> recipes (recipe_id));
joinable!(recipes -> users (user_id));

allow_tables_to_appear_in_same_query!(ingredients, instructions, recipes, users,);
