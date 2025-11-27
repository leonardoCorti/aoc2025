use std::{collections::HashMap, fs, path::Path, time::Instant};

use colored::*;
use log::debug;

pub type PartFn = fn(&str) -> String;

#[derive(Debug)]
pub struct Day {
    pub part1: PartFn,
    pub part2: PartFn,
}

#[derive(Debug)]
pub struct Registry {
    pub days: HashMap<&'static str, Day>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            days: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &'static str, part1: PartFn, part2: PartFn) {
        self.days.insert(name, Day { part1, part2 });
    }
}

pub fn get_input(day: &str) -> anyhow::Result<String> {
    let path = format!("input/{day}.txt");

    if Path::new(&path).exists() {
        return Ok(fs::read_to_string(path)?);
    }

    let session = std::env::var("AOC_SESSION").expect("Set AOC_SESSION environment variable!");

    let url = format!(
        "https://adventofcode.com/2024/day/{}/input",
        day.trim_start_matches("day"),
    );

    println!("Downloading input for {day}…");

    let text = reqwest::blocking::Client::new()
        .get(url)
        .header("Cookie", format!("session={session}"))
        .send()?
        .text()?;

    if text == "404 Not Found\n" {
        return Err(anyhow::anyhow!("Input not found"));
    }

    fs::create_dir_all("input")?;
    fs::write(&path, &text)?;

    Ok(text)
}

pub fn run_part(part: u8, partfn: PartFn, input: &str) {
    let start = Instant::now();
    let out = partfn(input);
    let t = start.elapsed();
    println!(
        "  Part {:<6}: {:<20}   ({:<10})",
        part.to_string().bold().red(),
        out.to_string().bold().yellow(),
        format!("{:?}", t).underline().green()
    );
}

pub fn run_day(day: &str, reg: &Registry, parts: Vec<u8>) -> anyhow::Result<()> {
    debug!("running day {day}, {parts:?}");
    let input = get_input(day)?;
    let entry = reg.days.get(day).expect("Day not registered");

    println!(
        "===================== {} =====================",
        day.to_string().italic().green()
    );

    for p in parts {
        match p {
            1 => run_part(1, entry.part1, &input),
            2 => run_part(2, entry.part2, &input),
            _ => panic!("Part must be 1 or 2"),
        }
    }

    Ok(())
}
