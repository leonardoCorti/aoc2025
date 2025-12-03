use log::debug;

// first implementation for part1
// fn calculate_joltage(input: &str) -> usize {
//     let numbers: Vec<usize> = input
//         .chars()
//         .map(|c| {
//             c.to_string()
//                 .parse::<usize>()
//                 .expect("could not parse a number")
//         })
//         .collect();
//
//     let idx = numbers[..(numbers.len() - 1)]
//         .iter()
//         .enumerate()
//         .fold(None, |acc, (i, v)| match acc {
//             None => Some((i, v)),
//             Some((best_i, best_v)) => {
//                 if v > best_v {
//                     Some((i, v))
//                 } else {
//                     Some((best_i, best_v))
//                 }
//             }
//         })
//         .map(|(i, _)| i)
//         .expect("could not find a maximum");
//
//     let idy = numbers
//         .iter()
//         .enumerate()
//         .skip(idx + 1)
//         .fold(None, |acc, (i, v)| {
//             match acc {
//                 None => Some((i, v)),
//                 Some((best_i, best_v)) => {
//                     if v > best_v {
//                         Some((i, v)) // nuovo massimo → prende il primo
//                     } else {
//                         Some((best_i, best_v))
//                     }
//                 }
//             }
//         })
//         .map(|(i, _)| i)
//         .expect("could not find a maximum");
//
//     debug!("il max x è in posizione {} ed è {}", idx, numbers[idx]);
//     debug!("il max y è in posizione {} ed è {}", idy, numbers[idy]);
//     debug!(
//         "quindi il risultat è {}",
//         (numbers[idx] * 10) + numbers[idy]
//     );
//     (numbers[idx] * 10) + numbers[idy]
// }

pub fn part1(input: &str) -> String {
    input
        .lines()
        // .map(calculate_joltage)
        .map(|e| calculate_joltage_n(e, 2))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    // input.len().to_string()
    input
        .lines()
        .map(|e| calculate_joltage_n(e, 12))
        .sum::<usize>()
        .to_string()
}

fn calculate_joltage_n(input: &str, n: usize) -> usize {
    debug!("input:\t {}", input);
    let numbers: Vec<usize> = input
        .chars()
        .map(|e| {
            e.to_string()
                .parse::<usize>()
                .expect("could not parse a number")
        })
        .collect();
    let mut digits_index: Vec<usize> = Vec::with_capacity(n);
    let mut last_digit_index = 0;
    for digit in (0..n).rev() {
        let index = numbers[..(numbers.len() - digit)]
            .iter()
            .enumerate()
            .skip(last_digit_index)
            .fold(None, |acc, (i, v)| {
                match acc {
                    None => Some((i, v)),
                    Some((best_i, best_v)) => {
                        if v > best_v {
                            Some((i, v)) // nuovo massimo → prende il primo
                        } else {
                            Some((best_i, best_v))
                        }
                    }
                }
            })
            .map(|(i, _)| i)
            .expect("could not find a maximum");
        last_digit_index = index + 1;
        debug!("{} at {digit}, it is the {digit} digit", numbers[index]);
        let searching_in: Vec<&usize> = numbers[..(numbers.len() - digit)]
            .iter()
            .skip(last_digit_index)
            .collect();
        debug!("searched in {:#?}", searching_in);
        debug!("skipped {last_digit_index}");
        digits_index.push(numbers[index]);
    }

    let result = vec_to_number(digits_index);
    debug!("result:\t{}", result);
    result
}

fn vec_to_number(digits: Vec<usize>) -> usize {
    let mut result = 0;
    for (index, num) in digits.iter().rev().enumerate() {
        if index == 0 {
            result += num;
        } else {
            result += ((10_isize.pow(index.try_into().unwrap())) * *num as isize) as usize;
        }
    }
    result
}

crate::generate_tests!(
    r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#,
    "357",
    "3121910778619"
);
