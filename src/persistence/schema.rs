table! {
    parents (id) {
        id -> Int4,
        parent -> Varchar,
        node -> Varchar,
    }
}
table! {
    nodes (id) {
        id -> Int4,
        node -> Varchar,
        visited -> Bool,
    }
}
