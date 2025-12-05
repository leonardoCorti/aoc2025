use crate::utils::*;
use log::debug;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Cell {
    #[default]
    Empty = 0,
    Roll = 1,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Roll => write!(f, "@"),
        }
    }
}

impl std::str::FromStr for Cell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Empty),
            "@" => Ok(Cell::Roll),
            _ => Err(format!("Invalid char: {}", s)),
        }
    }
}

pub fn part1(input: &str) -> String {
    let map: Grid<Cell> = Grid::from_ascii(input.trim()).expect("could not parse map");
    debug!("the map parsed is:\n{}", map);

    map.into_iter()
        .filter(|(_, cell)| **cell == Cell::Roll)
        .filter(|((x, y), _)| {
            map.neighbors8(*x, *y)
                .iter()
                .filter(|(_, n)| **n != Cell::Empty)
                .count()
                < 4
        })
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut map: Grid<Cell> = Grid::from_ascii(input.trim()).expect("could not parse map");
    debug!("the map parsed is:\n{}", map);
    let mut rolls_removed = 0;

    loop {
        let accessible_rolls: Vec<(usize, usize)> = map
            .into_iter()
            .filter(|(_, c)| **c == Cell::Roll)
            .filter(|((x, y), _)| {
                map.neighbors8(*x, *y)
                    .iter()
                    .filter(|(_, n)| **n == Cell::Roll)
                    .count()
                    < 4
            })
            .map(|(c, _)| c)
            .collect();

        if accessible_rolls.is_empty() {
            break;
        }

        rolls_removed += accessible_rolls.len();

        debug!("the coordinates of accessible rolls:{accessible_rolls:?}");

        for (x, y) in accessible_rolls {
            map.set(x, y, Cell::Empty);
        }

        debug!("map now is:\n{}", map);
    }

    rolls_removed.to_string()
}

crate::generate_tests!(
    r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#,
    "13",
    "43"
);
