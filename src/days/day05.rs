use log::debug;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> String {
    let (ranges, ingredients) = input.split_once("\n\n").expect("could not find empty line");
    debug!("examining {ranges}");
    let ranges: Vec<RangeInclusive<usize>> = ranges
        .lines()
        .map(|l| {
            let numbers = l.split_once("-").expect("did not found the - for range");
            debug!("{numbers:?}");
            numbers.0.parse::<usize>().expect("could not parse first")
                ..=numbers.1.parse::<usize>().expect("could not parse second")
        })
        .collect();
    debug!("examining {ingredients}");

    let mut ranges2 = ranges.clone();
    ranges2.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<usize>> = Vec::new();
    for range in ranges2 {
        if let Some(last) = merged.last_mut() {
            if *range.start() <= *last.end() + 1 {
                *last = *last.start()..=*last.end().max(range.end());
                continue;
            }
        }
        merged.push(range);
    }

    ingredients
        .lines()
        .map(|n| {
            n.parse::<usize>()
                .expect("could not parse number ingredient")
        })
        .filter(|n| {
            for single_range in &merged {
                if single_range.contains(&n) {
                    return true;
                }
            }
            return false;
        })
        .count()
        .to_string()

    // ingredients
    //     .lines()
    //     .map(|n| {
    //         n.parse::<usize>()
    //             .expect("could not parse number ingredient")
    //     })
    //     .filter(|n| {
    //         for single_range in &ranges {
    //             if single_range.contains(&n) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     })
    //     .count()
    //     .to_string()
}

pub fn part2(input: &str) -> String {
    let (ranges, _) = input.split_once("\n\n").expect("could not find empty line");
    let ranges: Vec<RangeInclusive<u128>> = ranges
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("-").expect("no - found");
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let mut ranges = ranges;
    ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<u128>> = Vec::new();
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if *range.start() <= *last.end() + 1 {
                *last = *last.start()..=*last.end().max(range.end());
                continue;
            }
        }
        merged.push(range);
    }

    let total: u128 = merged.iter().map(|r| r.end() - r.start() + 1).sum();

    total.to_string()
}

crate::generate_tests!(
    r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#,
    "3",
    "14"
);
