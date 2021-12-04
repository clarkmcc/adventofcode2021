#![allow(dead_code, unused_variables)]

/// This problem requires different forms of calculations depending on which bits are
/// the least common and which bits are the most common.
#[derive(Clone)]
enum Commonality {
    Most,
    Least,
}

/// This function calculates the power consumption of the submarine given a diagnostics
/// report. This was mainly an execise in bit shifting.
fn calculate_power_consumption(report: Vec<usize>, size: usize) -> usize {
    let bit_counts = get_bit_counts(&report, size);
    let result = bit_counts
        .1
        .iter()
        .enumerate()
        .fold((0b0, 0b0), |(g, e), (idx, c)| {
            (
                // Gamma looks for the most common bits
                get_common_mask(Commonality::Most, &bit_counts, size),
                // Epsilon looks for the least common bits
                get_common_mask(Commonality::Least, &bit_counts, size),
            )
        });
    result.0 * result.1
}

/// Calculates the Oxygen and CO2 rating from the given reports
pub fn calculate_oxygen_co2_rating(report: Vec<usize>, size: usize) -> usize {
    filter_on_commonality(Commonality::Most, report.clone(), size)
        * filter_on_commonality(Commonality::Least, report, size)
}

/// Filters reports based on bit commonality. Part #2 of this challenge requires that we filter each report
/// bit-by-bit from left-to-right based on whether the bit is the most common (for Oxygen) or the least common
/// (for CO2). This function takes the commonality that we want to use and filters the reports down by applying
/// the appropriate bit mask to each report.
fn filter_on_commonality(commonality: Commonality, report: Vec<usize>, size: usize) -> usize {
    let mut report = report;
    for n in 0..size {
        let bit_counts = get_bit_counts(&report, size);
        let mask = get_common_mask(commonality.clone(), &bit_counts, size);
        report = report
            .iter()
            // Perform witchcraft here which shifts to the bit whose index is 'n' starting from the left
            // and compares it to the respective bit in the mask, where the mask's respective bit indicates
            // whether or not that bit is the most common bit or not. If the two bits match then the bit
            // in our report must be the most common bit, therefore keep it.
            .filter(|v| (**v >> size - (n + 1) & 1) == (mask >> size - (n + 1) & 1))
            .map(|v| *v)
            .collect::<Vec<usize>>();
        if report.len() == 1 {
            break;
        }
    }
    *report.get(0).unwrap()
}

/// Returns the number of 0s and the number of 1s in each bit position of the entire report
/// vector. Each index of the returns 0s and 1s vector corresponds to a bit position in each
/// u64 value of report.
fn get_bit_counts(report: &Vec<usize>, size: usize) -> (Vec<usize>, Vec<usize>) {
    report.iter().fold(
        // These slices contain the number of binary ones and zeros from right-to-left like
        // binary is represented.
        (vec![0; size], vec![0; size]),
        |(mut zeros, mut ones), b| {
            let len = ones.len();
            for idx in 0..len {
                // Here we check each bit of the binary number 'b', if the bit is 1 then
                // we increment the ones slice value at the same index as the bit, other-
                // -wise we increment the zeros slice value at the same index as the bit.
                // Note that we're moving from right-to-left here as well.
                if (b >> (len - (idx + 1))) & 1 == 1 {
                    ones[idx] += 1;
                } else {
                    zeros[idx] += 1;
                }
            }
            (zeros, ones)
        },
    )
}

/// Returns a bit mask that indicates the most common bit at each bit position of the bit_counts
/// vector. Part #1 and part #2 of this problem require getting a bit mask depending on whether
/// we want to know the most or least common bits in each bit position.
fn get_common_mask(
    commonality: Commonality,
    bit_counts: &(Vec<usize>, Vec<usize>),
    size: usize,
) -> usize {
    let mut out = 0b0;
    for idx in 0..size {
        let commonality = match commonality {
            Commonality::Most => true,
            Commonality::Least => false,
        };
        let a = bit_counts.1[idx];
        let b = bit_counts.0[idx];
        if (a >= b) == commonality {
            out |= 1 << (size - (idx + 1))
        }
    }
    out
}

/// Parses the input dataset and returns a vector of 64-bit unsigned integers and the actual bit-size
/// that should be used for calculation. The actual bitsize is based on the number of bits that actually
/// contain useful values.
pub fn parse_input() -> (Vec<usize>, usize) {
    let mut size = 0;
    (
        include_str!("input.txt")
            .lines()
            .map(|v| {
                if size == 0 {
                    size = v.len();
                }
                usize::from_str_radix(v, 2).expect("should parse as base 2")
            })
            .collect::<Vec<usize>>(),
        size,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_power_consumption_example() {
        assert_eq!(
            198,
            calculate_power_consumption(
                vec![
                    0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100,
                    0b10000, 0b11001, 0b00010, 0b01010,
                ],
                5
            )
        );
    }

    #[test]
    fn test_calculate_oxygen_co2_ratings_example() {
        assert_eq!(
            230,
            calculate_oxygen_co2_rating(
                vec![
                    0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100,
                    0b10000, 0b11001, 0b00010, 0b01010,
                ],
                5,
            )
        );
    }

    #[test]
    fn test_calculate_oxygen_co2_ratings_input() {
        let (bytes, size) = parse_input();
        assert_eq!(4273224, calculate_oxygen_co2_rating(bytes, size,));
    }

    #[test]
    fn test_calculate_input() {
        let (bytes, size) = parse_input();
        assert_eq!(4138664, calculate_power_consumption(bytes, size));
    }

    #[test]
    fn test_get_most_common_mask() {
        let bit_counts = get_bit_counts(
            &vec![
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ],
            5,
        );
        assert_eq!(0b10110, get_common_mask(Commonality::Most, &bit_counts, 5));
        assert_eq!(0b01001, get_common_mask(Commonality::Least, &bit_counts, 5));
    }

    #[test]
    fn test_get_most_common_mask_same_number() {
        // Given the same number of 1s and 0s, 1s are given priority for [`Commonality::Most`] whereas
        // 0s are given priority for [`Commonality::Least`].
        let bit_counts = get_bit_counts(&vec![0b1, 0b0], 1);
        assert_eq!(0b1, get_common_mask(Commonality::Most, &bit_counts, 1));
        assert_eq!(0b0, get_common_mask(Commonality::Least, &bit_counts, 1));
    }
}
