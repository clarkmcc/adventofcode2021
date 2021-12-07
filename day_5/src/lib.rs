#![allow(dead_code, unused_variables)]

use itertools::iproduct;

/// When I originally wrote this solution, I used hashmaps to determine if we had lines overlapping
/// the same points. After loading all of the points into the hashmap, I would iterate over the it,
/// filtering out any points that had less than two overlapping lines. This turned out to be a lot
/// slower than I expected (about 45ms) so instead of iterating over the hashmap, I used the hashmap
/// to determine when a point had been overlapped two times and then incremented a counter. This
/// effectively removed the lookup iteration at the end, however I was still at about 15-20ms. The
/// next thing I tried was a highly purpose-built data structure ([`Grid`]) which would use vectors
/// for point overlap tracking and counters for tracking how often an overlap ocurred. I knew that
/// there would be a point at which a hashmap would be more effective than a vector, but the benchmarks
/// showed that at this scale (2d matrix with a width and height of 1000) the performance of a vector
/// was still faster than the hashmap. I was able to get it down to ~9ms using the vector.
///
/// The next optimizations will likely take place in the algorithm that produces points on a line.

/// Defines a point using X (0) and Y(1) values
#[derive(Clone, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    /// Creates a new point given an X and Y value
    fn from(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

/// Specifies a line defined by two points
#[derive(Clone)]
pub struct Line(Point, Point);

impl Line {
    /// Returns all the whole number points on a single line
    fn points(&self) -> Vec<Point> {
        if !self.diagonal() {
            iproduct!(range(self.0.x, self.1.x), range(self.0.y, self.1.y))
                .map(|(x, y)| Point::from(x, y))
                .collect::<Vec<Point>>()
        } else {
            range(self.0.x, self.1.x)
                .into_iter()
                .zip(range(self.0.y, self.1.y))
                .map(|(x, y)| Point::from(x, y))
                .collect()
        }
    }

    /// Returns true if the line is not a horizontal line or a vertical line
    fn diagonal(&self) -> bool {
        self.0.x != self.1.x && self.0.y != self.1.y
    }
}

struct Grid {
    // Conditionally calculating diagonals was able to save some computational overhead
    // when tackling part #1 of the problem.
    calculate_diagonals: bool,
    non_diagonal_rows: Vec<Vec<usize>>,
    non_diagonal_counter: usize,
    all_rows: Vec<Vec<usize>>,
    all_counter: usize,
}

impl Grid {
    fn with_size(size: usize, calculate_diagonals: bool) -> Self {
        let mut all_rows = Vec::with_capacity(size);
        for _ in 0..size {
            all_rows.push(vec![0; size]);
        }
        Self {
            calculate_diagonals,
            non_diagonal_rows: all_rows.clone(),
            non_diagonal_counter: 0,
            all_rows,
            all_counter: 0,
        }
    }

    /// Pushes a vector of lines into the grid and calculates the appropriate point overlaps for
    /// each line, saving the results into the [`Grid`]'s counters.
    fn push_lines(&mut self, lines: Vec<Line>) {
        lines.iter().for_each(|l| {
            l.points().iter().for_each(|p| {
                if !l.diagonal() {
                    self.non_diagonal_rows[p.x as usize][p.y as usize] += 1;
                    if self.non_diagonal_rows[p.x as usize][p.y as usize] == 2 {
                        self.non_diagonal_counter += 1;
                    }
                }
                if self.calculate_diagonals {
                    self.all_rows[p.x as usize][p.y as usize] += 1;
                    if self.all_rows[p.x as usize][p.y as usize] == 2 {
                        self.all_counter += 1;
                    }
                }
            })
        })
    }
}

/// Solves part #1 of the problem which asks for the number of overlapping points
/// from vertical or horizontal lines.
pub fn count_overlapping_points_part_1(lines: Vec<Line>) -> usize {
    let mut grid = Grid::with_size(1000, false);
    grid.push_lines(lines);
    grid.non_diagonal_counter
}

/// Solves part #2 of the problem which asks for the number of overlapping points
/// from horizontal, vertical, or diagonal lines.
pub fn count_overlapping_points_part_2(lines: Vec<Line>) -> (usize, usize) {
    let mut grid = Grid::with_size(1000, true);
    grid.push_lines(lines);
    (grid.all_counter, grid.non_diagonal_counter)
}

/// Returns a range of numbers between the two provided values. If the first value is higher
/// then the second value, then we produce a decreasing range.
fn range(a: isize, b: isize) -> Vec<isize> {
    if a > b {
        (b..a + 1).rev().collect::<Vec<isize>>()
    } else {
        (a..b + 1).collect::<Vec<isize>>()
    }
}

/// Parses the input.txt file and returns a vector of lines. This function can panic if the
/// input does not match the expected format.
pub fn parse_input() -> Vec<Line> {
    include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|l| {
            let mut points = l.split("->").into_iter().map(|v| {
                let mut parts = v.split(",").into_iter();
                Point {
                    x: parts
                        .next()
                        .expect("expected x")
                        .trim()
                        .parse::<isize>()
                        .expect("expected isize"),
                    y: parts
                        .next()
                        .expect("expected y")
                        .trim()
                        .parse::<isize>()
                        .expect("expected isize"),
                }
            });
            let first = points.next().expect("expected first point");
            let second = points.next().expect("expected second point");
            Line(first, second)
        })
        .collect::<Vec<Line>>()
}

#[test]
fn line_points() {
    let points = Line(Point::from(0, 8), Point::from(8, 0)).points();
    assert_eq!(9, points.len());
    let points = Line(Point::from(8, 0), Point::from(0, 8)).points();
    assert_eq!(9, points.len());
    let points = Line(Point::from(2, 2), Point::from(2, 1)).points();
    assert_eq!(2, points.len());
    let points = Line(Point::from(0, 9), Point::from(5, 9)).points();
    assert_eq!(6, points.len());
    let points = Line(Point::from(9, 4), Point::from(3, 4)).points();
    assert_eq!(7, points.len());
    let points = Line(Point::from(7, 0), Point::from(7, 4)).points();
    assert_eq!(5, points.len());
    let points = Line(Point::from(6, 4), Point::from(2, 0)).points();
    assert_eq!(5, points.len());
}

#[test]
fn test_range() {
    assert_eq!(vec![1, 2, 3], range(1, 3));
    assert_eq!(vec![3, 2, 1], range(3, 1));
    assert_eq!(vec![9], range(9, 9));
}

#[test]
fn test_count_overlapping_points_part_2() {
    let (all, non_diagonal) = count_overlapping_points_part_2(parse_input());
    assert_eq!(15463, all);
    assert_eq!(5698, non_diagonal);
}

#[test]
fn test_count_overlapping_points_part_1() {
    let count = count_overlapping_points_part_1(parse_input());
    assert_eq!(5698, count);
}
