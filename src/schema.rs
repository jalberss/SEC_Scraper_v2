table! {
    posts (id) {
        id -> Int4,
        acc_number -> Text,
    }
}

table! {
    test_posts (id) {
        id -> Int4,
        acc_number -> Text,
    }
}

allow_tables_to_appear_in_same_query!(posts, test_posts,);
