#[allow(dead_code)]
#[allow(unused)]
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub standings: Vec<Team>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    #[serde(deserialize_with = "extract_name")]
    pub team_name: String,
    pub games_played: u8,
    pub wins: u8,
    pub ties: u8,
    pub losses: u8,
    pub points: u8,
    pub conference_name: String,
    pub division_name: String,
    pub league_sequence: u8,
    pub conference_sequence: u8,
    pub division_sequence: u8,
}

fn extract_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct Name {
        default: String,
    }
    let helper = Name::deserialize(deserializer)?;
    Ok(helper.default)
}
