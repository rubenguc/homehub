// @generated automatically by Diesel CLI.

diesel::table! {
    home (id) {
        id -> Integer,
        street -> Text,
        number -> Text,
        floor -> Text,
        zipcode -> Text,
        squaremeters -> Text,
        number_of_bathrooms -> Text,
        number_of_rooms -> Text,
        home_type -> Text,
    }
}
