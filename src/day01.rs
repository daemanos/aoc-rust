pub struct Soln;

impl crate::Soln for Soln {
    type Answer = u32;

    fn part1(input: &str) -> Self::Answer {
        input.lines().map(extract_num).sum()
    }

    fn part2(input: &str) -> Self::Answer {
        let digits = vec![
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

        input.lines()
            .map(|line| {
                let d1 = digits
                    .iter()
                    .filter_map(|(pat, d)| line.find(pat).map(|idx| (idx, d)))
                    .min_by_key(|(idx, _)| idx.clone());
                let d2 = digits
                    .iter()
                    .filter_map(|(pat, d)| line.rfind(pat).map(|idx| (idx, d)))
                    .max_by_key(|(idx, _)| idx.clone());

                match (d1, d2) {
                    (Some((_, d)), None) => 10*d + d,
                    (Some((_, d1)), Some((_, d2))) => 10*d1 + d2,
                    _ => panic!(),
                }
            }).sum()
    }
}

fn extract_num(s: &str) -> u32 {
    let ds: Vec<u32> = s.trim().chars()
        .filter_map(|ch| ch.to_digit(10))
        .collect();

    let d1 = ds.first().copied().unwrap_or(0);
    let d2 = ds.last().copied().unwrap_or(d1);

    10 * d1 + d2
}
