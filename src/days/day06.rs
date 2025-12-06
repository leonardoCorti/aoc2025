use std::{fmt::Display, str::FromStr};

use itertools::Itertools;
use log::{debug, trace};

use crate::utils::Grid;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operator {
    Sum,
    Multiply,
}

impl Operator {
    fn calculate(&self, n1: usize, n2: usize) -> usize {
        match self {
            Operator::Sum => n1 + n2,
            Operator::Multiply => n1 * n2,
        }
    }
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Self::Sum),
            "*" => Ok(Self::Multiply),
            _ => Err("unknown operator {s}".to_string()),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Sum => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Element {
    Number(usize),
    Operator(Operator),
}

impl FromStr for Element {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(n) => Ok(Self::Number(n)),
            Err(_) => Ok(Self::Operator(Operator::from_str(s)?)),
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number(number) => write!(f, "{number:}\t"),
            Element::Operator(operator) => write!(f, "{operator:}\t"),
        }
    }
}

pub fn part1(input: &str) -> String {
    let map: Grid<Element> = Grid::from_ascii(input, None).expect("could not parse map");
    (0..map.width)
        .map(|i| map.iter_column(i))
        .map(|column| column.map(|(_, el)| el).collect::<Vec<&Element>>())
        .map(|column| {
            let operator: Operator = match column.last().unwrap() {
                Element::Number(_) => panic!("should be operator"),
                Element::Operator(operator) => *operator,
            };
            let mut accumulator = match operator {
                Operator::Sum => 0,
                Operator::Multiply => 1,
            };
            for n in &column[..(column.len() - 1)] {
                let num = match n {
                    Element::Number(nu) => nu,
                    Element::Operator(_) => panic!("should be a number"),
                };
                accumulator = operator.calculate(accumulator, *num);
            }
            debug!("for line {column:?} the result is {accumulator}");
            accumulator
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut columns: Vec<usize> = Vec::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().collect_vec().len();
    let mut operators: Vec<char> = Vec::new();
    for i in 0..width {
        let character = get(input, i, height - 1).unwrap_or(' ');
        debug!("{character}");
        if character != ' ' {
            columns.push(i);
            operators.push(character);
        }
    }
    let mut numbers: Vec<Vec<usize>> = Vec::with_capacity(columns.len());

    for index in 0..columns.len() {
        let mut this_row: Vec<usize> = Vec::new();
        let start_columns = columns[index];
        let mut last_column = if index != columns.len() - 1 {
            columns[index + 1]
        } else {
            width
        };
        trace!("start column is {start_columns} and end column is {last_column}");
        let mut how_many_numbers = last_column - start_columns;
        if index != columns.len() - 1 {
            how_many_numbers -= 1;
            last_column -= 1;
        }
        trace!("ci sono {how_many_numbers} numeri");

        for x in (start_columns..last_column).rev() {
            let mut this_column: Vec<usize> = Vec::new();
            for y in 0..(height - 1) {
                let character = get(input, x, y).expect("could not get a char");
                // debug!("analizzando {}", character);
                if character != ' ' {
                    let character_to_number: usize = character
                        .to_string()
                        .parse()
                        .expect("could not parse number");
                    this_column.push(character_to_number);
                }
            }
            let number_from_column = vec_to_number(this_column);
            debug!("the number is {number_from_column}");
            this_row.push(number_from_column);
        }

        numbers.push(this_row);
    }
    debug!("{numbers:?}");
    debug!("{operators:?}");
    let mut results: Vec<usize> = Vec::new();

    for (op, nums) in operators.iter().zip(numbers) {
        debug!("doing {op} on {nums:?}");
        let mut accumulator = match op {
            '*' => 1,
            '+' => 0,
            _ => panic!("unsupported operator"),
        };
        for n in nums {
            debug!("operating {accumulator} {op} {n}");
            accumulator = match op {
                '*' => accumulator * n,
                '+' => accumulator + n,
                _ => panic!("op not supported"),
            };
            debug!("operatred {accumulator}");
        }
        debug!("result is {accumulator}");
        results.push(accumulator);
    }

    debug!("{results:?}");
    results.iter().sum::<usize>().to_string()
}

fn get(input: &str, x: usize, y: usize) -> Option<char> {
    input
        .lines()
        .nth(y)
        .expect("couldn't get the line")
        .chars()
        .nth(x)
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
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#,
    "4277556",
    "3263827"
);
