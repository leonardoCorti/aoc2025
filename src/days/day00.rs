use log::debug;

pub fn part1(input: &str) -> String {
    debug!("debug in part1");
    // std::thread::sleep(std::time::Duration::from_micros(5));
    input.lines().count().to_string()
}

pub fn part2(input: &str) -> String {
    debug!("debug in part2");
    // std::thread::sleep(std::time::Duration::from_secs(1));
    input.len().to_string()
}

crate::generate_tests!(
    r#"
line1
line2
line3
"#,
    "3",
    "17"
);
