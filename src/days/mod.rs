use crate::framework::Registry;

macro_rules! register_day_and_module {
    ($( $daymod:ident ),+) => {
        $(
        pub mod $daymod;
        )+

        pub fn register_all(reg: &mut Registry) {
        $(
        let name = stringify!($daymod);
        let short = &name[3..];

        reg.register(short, $daymod::part1, $daymod::part2);
        )+
        }
    };
}

register_day_and_module!(day00, day01, day02, day03, day04, day05, day06);

#[macro_export]
macro_rules! generate_tests {
    // Single example for both parts
    ($example:expr, $part1_expected:expr, $part2_expected:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use std::sync::Once;

            static INIT: Once = Once::new();

            fn init_logger() {
                INIT.call_once(|| {
                    env_logger::builder().is_test(true).try_init().ok();
                });
            }

            const EXAMPLE: &str = $example;

            #[test]
            fn part1_example() {
                init_logger();
                let result = part1(EXAMPLE.trim());
                assert_eq!(result, $part1_expected);
            }

            #[test]
            fn part2_example() {
                init_logger();
                let result = part2(EXAMPLE.trim());
                assert_eq!(result, $part2_expected);
            }
        }
    };

    // Separate examples for part1 and part2
    ($example1:expr, $part1_expected:expr, $example2:expr, $part2_expected:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use std::sync::Once;

            static INIT: Once = Once::new();

            fn init_logger() {
                INIT.call_once(|| {
                    env_logger::builder().is_test(true).try_init().ok();
                });
            }

            const EXAMPLE1: &str = $example1;
            const EXAMPLE2: &str = $example2;

            #[test]
            fn part1_example() {
                init_logger();
                let result = part1(EXAMPLE1.trim());
                assert_eq!(result, $part1_expected);
            }

            #[test]
            fn part2_example() {
                init_logger();
                let result = part2(EXAMPLE2.trim());
                assert_eq!(result, $part2_expected);
            }
        }
    };
}
