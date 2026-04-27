mod tournament;
use clap::Parser;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use ron::ser::{PrettyConfig, to_string_pretty};
use ron::{Error as RonError, de::SpannedError};
use std::fmt::Display;
use std::io::{Error as IoError, ErrorKind as IoErrKind};
use thiserror::Error;

use crate::tournament::{Entrant, Event, Individual, Team, Tournament};

#[derive(Parser)]
pub enum Command {
    Reward {
        entrant: Entrant,
        name: String,
        points: i32,
    },
    Initalize,
    Display {
        name: String,
    },
    Rank {
        entrant: Entrant,
    },
}

fn main() -> Result<(), Error> {
    let command = Command::parse();
    let mut tournament = Tournament::from_file().or_else(|err| match err {
        Error::Io(err) if err.kind() == IoErrKind::NotFound => Ok(Tournament::default()),
        _ => Err(err),
    })?;
    if tournament.individuals.len() > 20 {
        return Err(Error::MaxIndividuals);
    }
    tournament.events.sort_by(Event::name_ord);
    tournament.events.dedup_by(Event::name_eq);
    tournament.individuals.sort_by(Individual::name_ord);
    tournament.individuals.dedup_by(Individual::name_eq);
    tournament.teams.sort_by(Team::name_ord);
    tournament.teams.dedup_by(Team::name_eq);
    match command {
        Command::Reward {
            entrant,
            name,
            points,
        } => match entrant {
            Entrant::Individual => {
                let index = tournament
                    .individuals
                    .binary_search_by_key(&name.as_str(), Individual::name)
                    .map_err(|_| Error::Missing(name))?;
                tournament.individuals[index].score = tournament.individuals[index]
                    .score
                    .saturating_add_signed(points);
            }
            Entrant::Team => {
                let index: usize = tournament
                    .teams
                    .binary_search_by_key(&name.as_str(), Team::name)
                    .map_err(|_| Error::Missing(name))?;
                tournament.teams[index].score =
                    tournament.teams[index].score.saturating_add_signed(points);
                println!("{}", points)
            }
        },
        Command::Initalize => {}
        Command::Display { name } => {
            let index: usize = tournament
                .events
                .binary_search_by_key(&name.as_str(), Event::name)
                .map_err(|_| Error::Missing(name))?;
            let event = to_string_pretty(&tournament.events[index], PrettyConfig::new())?;
            println!("{}", event);
        }
        Command::Rank { entrant } => match entrant {
            Entrant::Individual => {
                let mut individuals = tournament.individuals.clone();
                individuals.sort_by_key(Individual::score);
                let individuals = to_string_pretty(&individuals, PrettyConfig::new())?;
                println!("Individuals by ranking of scores {}", individuals)
            }
            Entrant::Team => {
                let mut teams = tournament.teams.clone();
                teams.sort_by_key(Team::score);
                let teams = to_string_pretty(&teams, PrettyConfig::new())?;
                println!("Teams by ranking of scores {}", teams)
            }
        },
    }
    tournament.to_file()
}

#[derive(Error)]
pub enum Error {
    #[error("Failed to read file {0}")]
    Io(#[from] IoError),
    #[error("Failed to read file {0}")]
    Ron(#[from] RonError),
    #[error("Failed to deserialize file {0}")]
    Spanned(#[from] SpannedError),
    #[error("Failed to find {0}")]
    Missing(String),
    #[error("Too many individuals")]
    MaxIndividuals,
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self, f)
    }
}
