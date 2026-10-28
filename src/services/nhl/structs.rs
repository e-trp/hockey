#[allow(dead_code)]
#[allow(unused)]
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use std::time::Duration;

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDetail {
    pub career_totals: PlayerCareer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCareer {
    pub regular_season: PlayerStatsSummary,
    pub playoffs: PlayerStatsSummary,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatsSummary {
    pub assists: u32,

    #[serde(deserialize_with = "deserialize_toi")]
    pub avg_toi: Duration,

    pub faceoff_winning_pctg: f64,
    pub game_winning_goals: u32,
    pub games_played: u32,
    pub goals: u32,
    pub ot_goals: u32,
    pub pim: u32,
    pub plus_minus: i32,
    pub points: u32,
    pub power_play_goals: u32,
    pub power_play_points: u32,
    pub shooting_pctg: f32,
    pub shorthanded_goals: u32,
    pub shorthanded_points: u32,
    pub shots: u32,
}

fn deserialize_toi<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    let (minutes, seconds) = value
        .split_once(':')
        .ok_or_else(|| D::Error::custom("Invalid TOI format"))?;

    let minutes: u64 = minutes.parse().map_err(D::Error::custom)?;
    let seconds: u64 = seconds.parse().map_err(D::Error::custom)?;

    if seconds >= 60 {
        return Err(D::Error::custom("Seconds must be less than 60"));
    }

    Ok(Duration::from_secs(minutes * 60 + seconds))
}