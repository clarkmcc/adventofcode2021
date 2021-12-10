/// Returns the cost to get all crab submarines to the optimal positions. The cost is calculated
/// by taking the mean (which is better than the median for this problem) and creating a window
/// before and after the mean. For each value in within this window, we calculate the optimal cost
/// and return that.
pub fn optimal_positions_part2(current: Vec<isize>, window: isize) -> isize {
    let avg = current.iter().sum::<isize>() / current.len() as isize;
    (avg - window..avg + window)
        .map(|pos| {
            current
                .iter()
                .map(|v| {
                    let s = (*v - pos).abs();
                    s * (s + 1) / 2
                })
                .sum::<isize>()
        })
        .min()
        .expect("expected a cost result")
}

/// Returns the cost to get all crab submarines to the optimal positions. It just so happens
/// that this can be calculated by positioning all submarines to the median and summing the
/// costs to get there. Getting the median is optimized using select_nth_unstable which performs
/// a quicksort. Honestly, I don't know why this works given that current's order matters and
/// select_nth_unstable cannot guaruntee that order.
pub fn optimal_positions_part1(mut current: Vec<isize>) -> isize {
    let mid = current.len() / 2;
    let med = *current.select_nth_unstable(mid).1;
    current.iter().map(|v| (*v - med).abs()).sum::<isize>()
}

// Parses the problem input
pub fn parse_input() -> Vec<isize> {
    include_str!("input.txt")
        .split(",")
        .map(|v| v.parse::<isize>().expect("expected usize"))
        .collect::<Vec<isize>>()
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
