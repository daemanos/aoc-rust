use crate::Soln;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = i64;

    fn part1(input: &str) -> Self::Answer {
        let races = Race::parse(input);
        races.iter()
            .map(Race::ways_to_win)
            .product()
    }

    fn part2(input: &str) -> Self::Answer {
        let input = input.replace(' ', "");
        let lines: Vec<&str> = input.lines().collect();

        let (_, time) = lines[0].split_once(':').unwrap();
        let time = time.parse().unwrap();

        let (_, dist) = lines[1].split_once(':').unwrap();
        let record = dist.parse().unwrap();

        let race = Race { time, record };
        race.ways_to_win()
    }
}

struct Race {
    time: f64,
    record: f64,
}

impl Race {
    fn parse(input: &str) -> Vec<Self> {
        let lines: Vec<&str> = input.lines().collect();

        let (_, times) = lines[0].split_once(':').unwrap();
        let times: Vec<f64> = times.trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let (_, dists) = lines[1].split_once(':').unwrap();
        let dists: Vec<f64> = dists.trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        times.iter().zip(dists)
            .map(|(&time, record)| Race { time, record })
            .collect()
    }

    fn ways_to_win(&self) -> i64 {
        let discrt = (self.time.powi(2) - 4.0*self.record).sqrt();
        let min = 1 + ((self.time - discrt) / 2.0).floor() as i64;
        let max = -1 + ((self.time + discrt) / 2.0).ceil() as i64;

        max - min + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "Time:      7  15   30
         Distance:  9  40  200";

    #[test]
    fn part1() {
        assert_eq!(288, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(71503, Puzzle::part2(&INPUT));
    }
}
