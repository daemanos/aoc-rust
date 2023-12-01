const RADIX: u32 = 10;
const DIGITS: [(&str, u32); 18] = [
    ("one", 1), ("1", 1),
    ("two", 2), ("2", 2),
    ("three", 3), ("3", 3),
    ("four", 4), ("4", 4),
    ("five", 5), ("5", 5),
    ("six", 6), ("6", 6),
    ("seven", 7), ("7", 7),
    ("eight", 8), ("8", 8),
    ("nine", 9), ("9", 9),
];

pub struct Soln;
impl crate::Soln for Soln {
    type Answer = u32;

    fn part1(input: &str) -> Self::Answer {
        input.lines()
            .map(|line| {
                let ds: Vec<u32> = line.trim().chars()
                    .filter_map(|ch| ch.to_digit(RADIX))
                    .collect();

                let d1 = ds.first().copied().unwrap_or(0);
                let d2 = ds.last().copied().unwrap_or(d1);

                RADIX * d1 + d2
            }).sum()
    }

    fn part2(input: &str) -> Self::Answer {
        input.lines()
            .map(|line| {
                let d1 = DIGITS.iter()
                    .filter_map(|(pat, d)| line.find(pat).map(|idx| (idx, d)))
                    .min_by_key(|(idx, _)| idx.clone())
                    .map(|(_, d)| d);
                let d2 = DIGITS.iter()
                    .filter_map(|(pat, d)| line.rfind(pat).map(|idx| (idx, d)))
                    .max_by_key(|(idx, _)| idx.clone())
                    .map(|(_, d)| d);

                match (d1, d2) {
                    (Some(d), None) => RADIX * d + d,
                    (Some(d1), Some(d2)) => RADIX * d1 + d2,
                    _ => panic!("no digits found in {line}"),
                }
            }).sum()
    }
}
