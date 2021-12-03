#![feature(test)]
#![allow(dead_code, unused_variables)]
extern crate test;
use std::io::{self, BufRead, Read};

/// Given a vector of measurements, this function returns the number of measurements that are higher
/// than the measurement immediately previous to them in the list. The first measurement is exempt
/// because there are no measurements before the first, and there will always be at least one measurement.
fn count_increasing_measurements(measurements: Vec<usize>) -> usize {
    // Starts as the first measurement, becomes the previous measurement at the end of each iteration
    let mut previous = measurements.get(0).unwrap();
    let mut out = 0;
    for next in measurements.iter().skip(1) {
        if next > previous {
            out += 1;
        }
        previous = next
    }
    out
}

/// Similar to count_increasing_measurements, this function groups the measurements in groups of `group_size`
/// before counting the number of increasing measurements. Each group is summed before comparison.
fn count_increasing_groups(measurements: Vec<usize>, group_size: usize) -> usize {
    let length = measurements.len();
    let mut groups: Vec<Vec<usize>> = vec![];
    for (idx, v) in measurements.iter().enumerate() {
        if idx + group_size > length {
            break;
        }
        groups.push(measurements[idx..idx + group_size].to_vec())
    }
    count_increasing_measurements(
        groups
            .iter()
            .map(|v| v.iter().map(|v| *v).reduce(|acc, next| acc + next))
            .map(|v| v.expect("expected successful reduction"))
            .collect::<Vec<usize>>(),
    )
}

/// Parses the input line by line and returns a vector of usize values. This function can panic and is
/// meant to be used in a controller environment such as a test.
fn parse_input<R: Read>(input: R) -> Vec<usize> {
    io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|v| {
            v.expect("expected line")
                .parse::<usize>()
                .expect("expected usize")
        })
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    use test::Bencher;

    #[test]
    fn example_part1_puzzle_input() {
        assert_eq!(
            7,
            count_increasing_measurements(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
        )
    }

    #[test]
    fn given_part1_puzzle_input() {
        let output = count_increasing_measurements(parse_input(BufReader::new(
            include_str!("input.txt").as_bytes(),
        )));
        assert_eq!(1696, output)
    }

    #[test]
    fn example_part2_puzzle_input() {
        let output =
            count_increasing_groups(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 3);
        assert_eq!(5, output);
    }

    #[test]
    fn given_part2_puzzle_input() {
        let output = count_increasing_groups(
            parse_input(BufReader::new(include_str!("input.txt").as_bytes())),
            3,
        );
        assert_eq!(1737, output);
    }

    #[bench]
    fn bench_count_increasing_measurements(b: &mut Bencher) {
        b.iter(|| {
            count_increasing_measurements(parse_input(BufReader::new(
                include_str!("input.txt").as_bytes(),
            )))
        });
    }

    #[bench]
    fn bench_count_increasing_groups(b: &mut Bencher) {
        b.iter(|| {
            count_increasing_groups(
                parse_input(BufReader::new(include_str!("input.txt").as_bytes())),
                3,
            )
        });
    }
}
