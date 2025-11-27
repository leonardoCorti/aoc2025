use log::debug;

pub fn part1(_input: &str) -> String {
    debug!("debug in part1");
    std::thread::sleep(std::time::Duration::from_micros(5));
    "part 1 of day00".to_string()
}

pub fn part2(_input: &str) -> String {
    debug!("debug in part2");
    std::thread::sleep(std::time::Duration::from_secs(1));
    "part 2 of day00".to_string()
}

#[cfg(test)]
mod tests {
    use log::debug;

    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            env_logger::builder().is_test(true).try_init().ok();
        });
    }

    const EXAMPLE1: &str = r#"
line1
line2
line3
"#;
    const EXAMPLE2: &str = r#"
line1
line2
line3
"#;

    #[test]
    fn part1_example() {
        init_logger();
        debug!("debug in test");
        let result = part1(EXAMPLE1.trim());
        assert_eq!(result, "3");
    }

    #[test]
    fn part2_example() {
        init_logger();
        let result = part2(EXAMPLE2.trim());
        assert_eq!(result, "15");
    }
}
