#![allow(dead_code, unused_variables)]

/// Returns the number of laternfish that would be alive after a number of days, given the initial
/// laternfish's ages.
pub fn num_laternfish(initial: Vec<usize>, days: usize) -> usize {
    let mut states = initial.iter().fold([0usize; 9], |mut m, v| {
        m[*v] += 1;
        m
    });
    (1..days).for_each(|d| states[(d + 7) % 9] += states[d % 9]);
    states.iter().sum()
}

/// Parses the problem input file
pub fn parse_input() -> Vec<usize> {
    include_str!("input.txt")
        .split(",")
        .map(|v| v.parse::<usize>().expect("expected usize"))
        .collect::<Vec<usize>>()
}

#[test]
fn test_num_laternfish_example() {
    assert_eq!(5934, num_laternfish(vec![3, 4, 3, 1, 2], 80));
}

#[test]
fn test_num_laternfish_input_part1() {
    assert_eq!(345793, num_laternfish(parse_input(), 80));
}

#[test]
fn test_num_laternfish_input_part2() {
    assert_eq!(1572643095893, num_laternfish(parse_input(), 256));
}
