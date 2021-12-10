/// Returns the cost to get all crab submarines to the optimal positions. The cost is calculated
/// by taking the mean (which is better than the median for this problem) and creating a window
/// before and after the mean. For each value in within this window, we calculate the optimal cost
/// and return that. This solution runs significantly faster than the part 1 solution because
/// calculating a mean does not require sorting.
pub fn optimal_positions_part2(current: Vec<usize>, window: usize) -> usize {
    let avg = current.iter().sum::<usize>() / current.len();
    (avg - window..avg + window)
        .map(|pos| {
            current
                .iter()
                .map(|v| {
                    let s = (*v as isize - pos as isize).abs();
                    s * (s + 1) / 2
                })
                .sum::<isize>() as usize
        })
        .reduce(|acc, next| if next < acc { next } else { acc })
        .expect("expected a cost result")
}

/// Returns the cost to get all crab submarines to the optimal positions. It just so happens
/// that this can be calculated by positioning all submarines to the median and summing the
/// costs to get there.
pub fn optimal_positions_part1(mut current: Vec<usize>) -> usize {
    // todo: try quickselect algorithm to avoid a sort
    current.sort();
    current
        .iter()
        .map(|v| {
            (*v as isize - *current.get(current.len() / 2).expect("expected a median") as isize)
                .abs() as usize
        })
        .sum::<usize>()
}

// Parses the problem input
pub fn parse_input() -> Vec<usize> {
    include_str!("input.txt")
        .split(",")
        .map(|v| v.parse::<usize>().expect("expected usize"))
        .collect::<Vec<usize>>()
}

#[test]
fn test_optimal_positions_example() {
    assert_eq!(
        37,
        optimal_positions_part1(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14])
    )
}

#[test]
fn test_optimal_positions_part1_input() {
    assert_eq!(359648, optimal_positions_part1(parse_input()))
}

#[test]
fn test_optimal_positions_part2_example() {
    assert_eq!(
        168,
        optimal_positions_part2(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 2)
    )
}

#[test]
fn test_optimal_positions_part2_input() {
    assert_eq!(100727924, optimal_positions_part2(parse_input(), 1))
}
