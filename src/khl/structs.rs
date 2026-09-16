use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Table {
    pub devisions: Vec<Division>,
}

#[derive(Debug)]
pub struct Division {
    pub name: String,
    pub teams: Vec<Team>,
}

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub clubid: u32,
    pub link: String,
}

impl<'de> Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Root {
            data: Data,
        }

        #[derive(Deserialize)]
        struct Data {
            json: Json,
        }

        #[derive(Deserialize)]
        struct Json {
            divisions: Vec<RawDivision>,
        }

        #[derive(Deserialize)]
        struct RawDivision {
            item: String,
            rows: Vec<RawTeam>,
        }

        #[derive(Deserialize)]
        struct RawTeam {
            name: String,
            clubid: u32,
            link: String,
        }

        let root = Root::deserialize(deserializer)?;

        Ok(Table {
            devisions: root
                .data
                .json
                .divisions
                .into_iter()
                .map(|division| Division {
                    name: division.item,
                    teams: division
                        .rows
                        .into_iter()
                        .map(|team| Team {
                            name: team.name,
                            clubid: team.clubid,
                            link: team.link,
                        })
                        .collect(),
                })
                .collect(),
        })
    }
}
