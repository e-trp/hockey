use diesel::prelude::*;

use crate::database::schema::{
    leagues,
    partitions,
    stats,
    teams,
    trophies,
};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = leagues)]
pub struct League {
    pub id: Option<i32>,
    pub ext_id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = partitions)]
pub struct Partition {
    pub id: Option<i32>,
    pub ext_id: i32,
    pub name: String,
    pub parent_id: i32,
    pub league_id: i32,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = stats)]
pub struct Stat {
    pub id: Option<i32>,
    pub team_id: i32,
    pub year: String,
    pub win: i32,
    pub lose: i32,
    pub draw: i32,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: Option<i32>,
    pub ext_id: i32,
    pub version: i32,
    pub create_time: String,
    pub delete_time: Option<String>,
    pub name: String,
    pub city: String,
    pub partition_id: i32,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = trophies)]
pub struct Trophy {
    pub id: Option<i32>,
    pub team_id: i32,
    pub name: String,
}
