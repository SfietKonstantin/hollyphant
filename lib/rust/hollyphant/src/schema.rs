// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        name -> Text,
        mas_instance -> Nullable<Text>,
        mas_client_id -> Nullable<Text>,
        mas_client_secret -> Nullable<Text>,
        mas_access_token -> Nullable<Text>,
    }
}
