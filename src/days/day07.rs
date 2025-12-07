use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use log::{debug, trace};

use crate::utils::Grid;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
enum Cell {
    #[default]
    Empty,
    Splitter,
    EntryPoint,
    ActiveBeam,
    Beam,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Splitter => write!(f, "^"),
            Cell::EntryPoint => write!(f, "S"),
            Cell::Beam => write!(f, "|"),
            Cell::ActiveBeam => write!(f, "A"),
        }
    }
}

impl std::str::FromStr for Cell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Empty),
            "^" => Ok(Cell::Splitter),
            "S" => Ok(Cell::EntryPoint),
            "|" => Ok(Cell::Beam),
            "A" => Ok(Cell::ActiveBeam),
            _ => Err(format!("Invalid char: {}", s)),
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut map: Grid<Cell> = Grid::from_chars(input).expect("could not parse map");
    let mut counter = 0;
    debug!("\n{map}");

    //create the first beam
    let starting_point = map
        .into_iter()
        .find(|(_, c)| Cell::EntryPoint == **c)
        .map(|(c, _)| c)
        .expect("could not find startingPoint");
    map.set(starting_point.0, starting_point.1 + 1, Cell::ActiveBeam);
    debug!("\n{map}");

    loop {
        //find the beams that can go down
        let active_beams: Vec<(usize, usize)> = map
            .into_iter()
            .filter(|(_, c)| **c == Cell::ActiveBeam)
            .map(|(c, _)| c)
            .collect();

        for (x, y) in &active_beams {
            map.set(*x, *y, Cell::Beam);
        }

        trace!("the active beams are :{active_beams:?}, the counter is {counter}");
        //if there are none break
        if active_beams.is_empty() {
            break;
        }
        //make the beam go on and update the counter
        for (x, y) in &active_beams {
            let y = y + 1;
            if y == map.height {
                debug!("reached end of the map");
                break;
            }
            debug!("inspecting {},{}", x, y - 1);
            match map.get(*x, y) {
                Cell::Empty => {
                    map.set(*x, y, Cell::ActiveBeam);
                }
                Cell::Splitter => {
                    map.set(x - 1, y, Cell::ActiveBeam);
                    map.set(x + 1, y, Cell::ActiveBeam);
                    counter += 1;
                }
                Cell::ActiveBeam => {}
                _ => panic!("beam should not encounter other Beams or EntryPoint"),
            }
        }
        //print
        debug!("\n{map}");
    }
    debug!("\n{map}");

    counter.to_string()
}

pub fn part2(input: &str) -> String {
    let mut map: Grid<Cell> = Grid::from_chars(input).expect("could not parse map");
    let starting_point = map
        .into_iter()
        .find(|(_, c)| Cell::EntryPoint == **c)
        .map(|(c, _)| c)
        .expect("could not find startingPoint");
    map.set(starting_point.0, starting_point.1 + 1, Cell::ActiveBeam);
    debug!("\n{map}");

    how_many_splits_encountered(map, (starting_point.0, starting_point.1 + 1)).to_string()
}

fn how_many_splits_encountered(map: Grid<Cell>, active_beam: (usize, usize)) -> usize {
    fn inner(
        map: &Grid<Cell>,
        active_beam: (usize, usize),
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(&cached) = memo.get(&(active_beam)) {
            return cached;
        }

        // If the beam reaches the bottom, count this path
        if active_beam.1 + 1 == map.height {
            memo.insert(active_beam, 1);
            return 1;
        }

        let next = map.get(active_beam.0, active_beam.1 + 1);

        // Normal forward progress
        if next == Cell::Empty {
            let mut next_map = map.clone();
            next_map.set(active_beam.0, active_beam.1, Cell::Beam);
            next_map.set(active_beam.0, active_beam.1 + 1, Cell::ActiveBeam);

            let res = inner(&next_map, (active_beam.0, active_beam.1 + 1), memo);
            memo.insert(active_beam, res);
            return res;
        }

        // Split: go left
        let mut left_map = map.clone();
        left_map.set(active_beam.0, active_beam.1, Cell::Beam);
        left_map.set(active_beam.0 - 1, active_beam.1 + 1, Cell::ActiveBeam);
        let left = inner(&left_map, (active_beam.0 - 1, active_beam.1 + 1), memo);

        // Split: go right
        let mut right_map = map.clone();
        right_map.set(active_beam.0, active_beam.1, Cell::Beam);
        right_map.set(active_beam.0 + 1, active_beam.1 + 1, Cell::ActiveBeam);
        let right = inner(&right_map, (active_beam.0 + 1, active_beam.1 + 1), memo);

        let total = left + right;
        memo.insert(active_beam, total);
        total
    }

    // top-level wrapper holding the memo map
    let mut memo = HashMap::new();
    inner(&map, active_beam, &mut memo)
}

crate::generate_tests!(
    r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#,
    "21",
    "40"
);
