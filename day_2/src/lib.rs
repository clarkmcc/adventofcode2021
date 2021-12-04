#![allow(dead_code, unused_variables)]

use std::{
    io::BufRead,
    io::{self, Read},
};

/// Specifies a movement direction and an magnitude moved in that direction for the
/// submarine. Because this is a submarine, down would increase the depth further
/// whereas up would decrease the depth.
enum Direction {
    // Increases our horizontal position
    Forward(usize),
    // Increases the depth
    Down(usize),
    // Decreases the depth
    Up(usize),
}

/// Calculates the final position given a vector of [`Direction`]. The final position
/// is determined by applying the usize contained in the direction to a horizonal position
/// or to the depth (depending on the direction). The final value is determined by
/// multiplying the horizontal position by the depth.
fn calculate_final_position(movements: Vec<Direction>) -> usize {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for movement in movements.iter() {
        match movement {
            Direction::Forward(n) => horizontal_position += n,
            Direction::Down(n) => depth += n,
            Direction::Up(n) => depth -= n,
        }
    }
    horizontal_position * depth
}

/// Similar to [`calculate_final_position`], this function takes into account the additional
/// instructions of tracking aim and determining the depth based on the aim. The additional
/// instructions came as part 2 of the problem.
fn calculate_final_position_part_2(movements: Vec<Direction>) -> usize {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for movement in movements.iter() {
        match movement {
            Direction::Forward(n) => {
                horizontal_position += n;
                depth += aim * n;
            }
            Direction::Down(n) => aim += n,
            Direction::Up(n) => aim -= n,
        }
    }
    horizontal_position * depth
}

/// Parses the input line by line and returns a vector of directions and magnitudes. This function can panic
/// and is meant to be used in a controlled environment such as a test.
fn parse_input<R: Read>(input: R) -> Vec<Direction> {
    io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|v| {
            let parts: Vec<String> = v
                .expect("expected line")
                .split_whitespace()
                .map(|v| v.to_string())
                .collect();
            let magnitude = parts
                .get(1)
                .expect("expected magnitude")
                .parse::<usize>()
                .unwrap();
            match parts.get(0).expect("expected direction").as_str() {
                "forward" => Direction::Forward(magnitude),
                "down" => Direction::Down(magnitude),
                "up" => Direction::Up(magnitude),
                _ => panic!("unrecognized direction"),
            }
        })
        .collect::<Vec<Direction>>()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_calculate_final_position() {
        assert_eq!(
            150,
            calculate_final_position(vec![
                Direction::Forward(5),
                Direction::Down(5),
                Direction::Forward(8),
                Direction::Up(3),
                Direction::Down(8),
                Direction::Forward(2),
            ])
        )
    }

    #[test]
    fn test_calculate_final_position_part_2() {
        assert_eq!(
            900,
            calculate_final_position_part_2(vec![
                Direction::Forward(5),
                Direction::Down(5),
                Direction::Forward(8),
                Direction::Up(3),
                Direction::Down(8),
                Direction::Forward(2),
            ])
        )
    }

    #[test]
    fn test_given_calculate_final_position() {
        assert_eq!(
            2117664,
            calculate_final_position(parse_input(BufReader::new(
                include_str!("input.txt").as_bytes(),
            )))
        )
    }

    #[test]
    fn test_given_calculate_final_position_part_2() {
        assert_eq!(
            2073416724,
            calculate_final_position_part_2(parse_input(BufReader::new(
                include_str!("input.txt").as_bytes(),
            )))
        )
    }
}
