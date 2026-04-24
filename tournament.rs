use clap::ValueEnum;
use std::{
    cmp::Ordering,
    fs::{read_to_string, write},
};

use crate::Error;
use ron::{
    from_str,
    ser::{PrettyConfig, to_string_pretty},
};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Tournament {
    pub events: Vec<Event>,
    pub individuals: Vec<Individaul>,
    pub teams: Vec<Team>,
}
impl Tournament {
    const PATH: &str = "tournament.ron";
    pub fn from_file() -> Result<Self, Error> {
        Ok(from_str(read_to_string(Self::PATH)?.as_str())?)
    }
    pub fn to_file(&self) -> Result<(), Error> {
        Ok(write(
            Self::PATH,
            to_string_pretty(&self, PrettyConfig::new())?.as_str(),
        )?)
    }
}
impl Default for Tournament {
    fn default() -> Self {
        Tournament {
            events: vec![
                Event {
                    name: "event_1".into(),
                    entrant: Entrant::Individual,
                    challenge: "sporting".into(),
                    participtants: vec!["individual".into()],
                },
                Event {
                    name: "event_2".into(),
                    entrant: Entrant::Individual,
                    challenge: "academic".into(),
                    participtants: vec!["team".into()],
                },
            ],
            individuals: vec![Individaul {
                name: "individual".into(),
                score: 0,
            }],
            teams: vec![Team {
                name: "team".into(),
                members: vec!["team_member".into()],
                score: 0,
            }],
        }
    }
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Event {
    pub name: String,
    pub entrant: Entrant,
    pub participtants: Vec<String>,
    pub challenge: String,
}
impl Event {
    pub fn name_ord(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
    pub fn name_eq(&mut self, other: &mut Self) -> bool {
        self.name == other.name
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Serialize, Deserialize, Default, Copy, Clone, ValueEnum, Debug)]
pub enum Entrant {
    #[default]
    Individual,
    Team,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Individaul {
    pub name: String,
    pub score: u32,
}
impl Individaul {
    pub fn name_ord(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
    pub fn name_eq(&mut self, other: &mut Self) -> bool {
        self.name == other.name
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn score(&self) -> u32 {
        self.score
    }
}
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Team {
    pub name: String,
    pub members: Vec<String>,
    pub score: u32,
}
impl Team {
    pub fn name_ord(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
    pub fn name_eq(&mut self, other: &mut Self) -> bool {
        self.name == other.name
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn score(&self) -> u32 {
        self.score
    }
}
