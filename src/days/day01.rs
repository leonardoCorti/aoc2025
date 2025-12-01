use log::debug;

#[derive(Debug)]
enum Rotation {
    Left(isize),
    Right(isize),
}

pub fn part1(input: &str) -> String {
    debug!("debug in part1");
    let rotations = parse_input(input);
    debug!("{:?}", rotations);
    let mut current_value: isize = 50;
    let mut counter: isize = 0;
    for rotation in rotations {
        match rotation {
            Rotation::Left(steps) => {
                current_value -= steps;
                while current_value < 0 {
                    current_value += 100;
                }
            }
            Rotation::Right(steps) => {
                current_value += steps;
                current_value %= 100;
            }
        }

        debug!(
            "the rotation is {:?} and the value is {}",
            rotation, current_value
        );

        if current_value == 0 {
            counter += 1;
        }
    }
    counter.to_string()
}

fn parse_input(input: &str) -> Vec<Rotation> {
    let rotations: Vec<Rotation> = input
        .lines()
        .map(|text| {
            let direction = &text[..1];
            let steps = &text[1..];
            let value: isize = steps.parse().expect("could not parse the number");
            match direction {
                "L" => Rotation::Left(value),
                "R" => Rotation::Right(value),
                _ => panic!("could not parse direction"),
            }
        })
        .collect();
    rotations
}

pub fn part2(input: &str) -> String {
    let rotations = parse_input(input);
    let mut current_value: isize = 50;
    let mut counter: isize = 0;
    for rotation in rotations {
        match rotation {
            Rotation::Left(steps) => {
                if steps < current_value {
                    current_value -= steps;
                    if current_value == 0 {
                        counter += 1;
                    }
                } else {
                    if current_value == 0 {
                        counter += steps / 100;
                    } else {
                        counter += 1 + (steps - current_value) / 100;
                    }
                    current_value = (current_value - steps).rem_euclid(100);
                }
            }
            Rotation::Right(steps) => {
                current_value += steps;
                while current_value >= 100 {
                    current_value -= 100;
                    counter += 1;
                }
            }
        }

        debug!(
            "the rotation is {:#?} and the value is {}, the counter is {counter}",
            rotation, current_value
        );
    }
    counter.to_string()
}

crate::generate_tests!(
    r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#,
    "3",
    "6"
);
