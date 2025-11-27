mod days;
mod framework;

use framework::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut reg = Registry::new();
    days::register_all(&mut reg);

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!("Usage: aoc 01[:1|:2][,02[:1|:2] ...]");
        return Ok(());
    }

    let mut tasks = Vec::new();

    let joined = args.join(",");
    for item in joined.split(',') {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }

        if let Some((day, part)) = item.split_once(':') {
            let p = part.parse::<u8>().expect("Part must be integer 1 or 2");
            tasks.push((day.to_string(), vec![p]));
        } else {
            tasks.push((item.to_string(), vec![1, 2]));
        }
    }

    for (day, parts) in tasks {
        run_day(&day, &reg, parts)?;
    }

    Ok(())
}
