table! {
    ingredients (id) {
        id -> Int4,
        recipe_id -> Int4,
        ingredient -> Text,
    }
}

table! {
    instructions (id) {
        id -> Int4,
        recipe_id -> Int4,
        step_number -> Int4,
        instruction -> Text,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        title -> Varchar,
    }
}

joinable!(ingredients -> recipes (recipe_id));
joinable!(instructions -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(ingredients, instructions, recipes,);
