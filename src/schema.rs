table! {
    auth_infos (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Text,
        mfa_enabled -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        dob -> Text,
        kyc_level -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    auth_infos,
    users,
);
