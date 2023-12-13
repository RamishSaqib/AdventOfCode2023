use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use anyhow::{anyhow, bail, Context, Result};

/// The maximum cube set i.e., the number of cubes of each color that is
/// contained in the bag for the puzzle.
const MAX_CUBE_SET: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

/// Represents a set of cubes of different colors.
#[derive(Debug, Default)]
struct CubeSet {
    /// The number of red cubes.
    red: u32,
    /// The number of green cubes.
    green: u32,
    /// The number of blue cubes.
    blue: u32,
}

impl CubeSet {
    /// Returns a new cube set with the given red count.
    fn with_red(mut self, red: u32) -> Self {
        self.red = red;
        self
    }

    /// Returns a new cube set with the given green count.
    fn with_green(mut self, green: u32) -> Self {
        self.green = green;
        self
    }

    /// Returns a new cube set with the given blue count.
    fn with_blue(mut self, blue: u32) -> Self {
        self.blue = blue;
        self
    }

    /// Returns `true` if the cube set is possible. That is, if the cube set
    /// does not exceed the maximum cube set.
    fn is_possible(&self) -> bool {
        self.red <= MAX_CUBE_SET.red
            && self.green <= MAX_CUBE_SET.green
            && self.blue <= MAX_CUBE_SET.blue
    }

    /// Returns the power of the cube set.
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_set = Self::default();
        for cube_info in s.split(", ") {
            let (count, color) = cube_info
                .split_once(' ')
                .ok_or_else(|| anyhow!("Invalid cube info: {:?}", cube_info))?;
            let count = count.parse::<u32>()?;
            cube_set = match color {
                "red" => cube_set.with_red(count),
                "green" => cube_set.with_green(count),
                "blue" => cube_set.with_blue(count),
                _ => bail!("Invalid color: {:?}", color),
            };
        }
        Ok(cube_set)
    }
}

/// Represents a single game of cube sets.
#[derive(Debug)]
struct Game {
    /// The ID of the game.
    id: u32,
    /// The cube sets in the game.
    sets: Vec<CubeSet>,
}

impl Game {
    /// Returns `true` if the game is possible.
    fn is_possible(&self) -> bool {
        self.sets.iter().all(CubeSet::is_possible)
    }

    /// Returns the minimum cube set required for this game to be possible.
    fn min_cube_set(&self) -> CubeSet {
        self.sets
            .iter()
            .fold(CubeSet::default(), |acc, cube_set| CubeSet {
                red: acc.red.max(cube_set.red),
                green: acc.green.max(cube_set.green),
                blue: acc.blue.max(cube_set.blue),
            })
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_section, cube_sets_section) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Expected a colon in the line"))?;

        let game_id = game_section
            .split_whitespace()
            .nth(1)
            .and_then(|id| id.trim_end_matches(':').parse::<u32>().ok())
            .ok_or_else(|| anyhow!("Invalid game section of the line: {:?}", game_section))?;

        let cube_sets = cube_sets_section
            .split("; ")
            .map(CubeSet::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            id: game_id,
            sets: cube_sets,
        })
    }
}

/// Represents a collection of games.
#[derive(Debug, Default)]
struct Games(Vec<Game>);

impl Games {
    /// Returns an iterator over the IDs of the games that are possible.
    fn possible_game_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.0
            .iter()
            .filter(|game| game.is_possible())
            .map(|game| game.id)
    }

    /// Returns an iterator over the powers of the minimum cube set for each
    /// game.
    fn min_cube_set_powers(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().map(|game| game.min_cube_set().power())
    }
}

impl FromStr for Games {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .enumerate()
                .map(|(index, line)| {
                    Game::from_str(line)
                        .with_context(|| format!("Failed to parse line {}: {:?}", index + 1, line))
                })
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let games = Games::from_str(input)?;

    println!("Part 1: {:?}", games.possible_game_ids().sum::<u32>());
    println!("Part 2: {:?}", games.min_cube_set_powers().sum::<u32>());

    Ok(())
}

fn main() {
    let mut data_file = File::open("src/advent_of_code.txt").unwrap();

    let mut file_content = String::new();

    data_file.read_to_string(&mut file_content).unwrap();

    solve(&file_content).expect("TODO: panic message");
}