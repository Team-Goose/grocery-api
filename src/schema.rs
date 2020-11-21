table! {
    account (id) {
        id -> Int4,
        username -> Varchar,
        pass -> Varchar,
        list -> Array<Int4>,
        friends -> Array<Int4>,
    }
}
