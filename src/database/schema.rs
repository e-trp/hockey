// @generated automatically by Diesel CLI.

diesel::table! {
    leagues (id) {
        id -> Nullable<Integer>,
        ext_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    partitions (id) {
        id -> Nullable<Integer>,
        ext_id -> Integer,
        name -> Text,
        parent_id -> Integer,
        league_id -> Integer,
    }
}

diesel::table! {
    stats (id) {
        id -> Nullable<Integer>,
        team_id -> Integer,
        year -> Text,
        win -> Integer,
        lose -> Integer,
        draw -> Integer,
    }
}

diesel::table! {
    teams (id) {
        id -> Nullable<Integer>,
        ext_id -> Integer,
        version -> Integer,
        create_time -> Text,
        delete_time -> Nullable<Text>,
        name -> Text,
        city -> Text,
        partition_id -> Integer,
    }
}

diesel::table! {
    trophies (id) {
        id -> Nullable<Integer>,
        team_id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(partitions -> leagues (league_id));
diesel::joinable!(stats -> teams (team_id));
diesel::joinable!(teams -> partitions (partition_id));
diesel::joinable!(trophies -> teams (team_id));

diesel::allow_tables_to_appear_in_same_query!(
    leagues,
    partitions,
    stats,
    teams,
    trophies,
);
