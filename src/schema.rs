// @generated automatically by Diesel CLI.

diesel::table! {
    book (book_id) {
        book_id -> Nullable<Integer>,
        book_name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    course (course_id) {
        course_id -> Nullable<Integer>,
        course_name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    ingredient (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    recipe (recipe_id) {
        recipe_id -> Nullable<Integer>,
        primary_season -> Integer,
        course -> Integer,
        book -> Nullable<Integer>,
        recipe_name -> Nullable<Text>,
        recipe_url -> Nullable<Text>,
        created_at -> Nullable<Float>,
        page -> Nullable<Integer>,
    }
}

diesel::table! {
    recipe_ingredient (recipe_id, ingredient_id) {
        recipe_id -> Integer,
        ingredient_id -> Integer,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    season (season_id) {
        season_id -> Nullable<Integer>,
        tag_name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::joinable!(recipe -> book (book));
diesel::joinable!(recipe -> course (course));
diesel::joinable!(recipe -> season (primary_season));
diesel::joinable!(recipe_ingredient -> ingredient (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    book,
    course,
    ingredient,
    recipe,
    recipe_ingredient,
    season,
);
