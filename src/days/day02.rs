use std::ops::Range;

use log::{debug, trace};

fn parse_all(input: &str) -> Vec<Range<usize>> {
    let mut result: Vec<Range<usize>> = Vec::new();
    for a_range in input.split(",") {
        trace!("splitting {a_range}");
        let (start, end) = a_range.split_once("-").expect("could not split the range");
        result.push(
            start
                .trim()
                .parse::<usize>()
                .expect("could not parse number")
                ..end.trim().parse::<usize>().expect("could not parse number") + 1,
        );
    }
    result
}

fn count_invalid_id(input: &Range<usize>) -> usize {
    let mut count = 0;
    debug!("the range {input:?}");
    let input = input.clone();
    for n in input {
        if n == 1188511885 {
            debug!("{n} raggiunto");
        }
        if !check_valid_id(n) {
            count += n;
        }
    }
    debug!(" has count {count}");
    count
}

fn check_valid_id(input: usize) -> bool {
    trace!("checking {input}");
    let input_string = input.to_string();
    let clean_input = input_string.trim_start_matches('0');
    let input_len = clean_input.len();
    let half_len = input_len / 2;
    if !input_len.is_multiple_of(2) {
        return true;
    };
    if clean_input[..half_len] != clean_input[half_len..] {
        return true;
    }
    debug!("{input} is invalid");
    false
}

pub fn part1(input: &str) -> String {
    let ranges = parse_all(input);
    debug!("{:?}", ranges);
    ranges
        .iter()
        .map(count_invalid_id)
        .sum::<usize>()
        .to_string()
}

fn count_invalid_id_v2(input: &Range<usize>) -> usize {
    let mut count = 0;
    debug!("the range {input:?}");
    let input = input.clone();
    for n in input {
        if n == 1188511885 {
            debug!("{n} raggiunto");
        }
        if !check_valid_id_v2(n) {
            count += n;
        }
    }
    debug!(" has count {count}");
    count
}

fn check_valid_id_v2(input: usize) -> bool {
    trace!("checking {input}");
    let input_string = input.to_string();
    let clean_input = input_string.trim_start_matches('0');
    let input_len = clean_input.len();

    for i in 1..input_len {
        if !input_len.is_multiple_of(i) {
            continue;
        }

        let sub = &clean_input[..i];
        let repetead = sub.repeat(input_len / i);
        if repetead == clean_input {
            debug!("{input} is invalid");
            return false;
        }
    }

    true
}

pub fn part2(input: &str) -> String {
    let ranges = parse_all(input);
    ranges
        .iter()
        .map(count_invalid_id_v2)
        .sum::<usize>()
        .to_string()
}

crate::generate_tests!(
    r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"#,
    "1227775554",
    "4174379265"
);
