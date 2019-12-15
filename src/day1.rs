use std::cmp;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn fuel_req(x: i32) -> i32 { (x / 3) - 2 }

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().fold(0, |sum, x| sum + fuel_req(*x))
}

fn fuel_fuel_req(x: i32) -> i32 { cmp::max(fuel_req(x), 0) }

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    input.iter()
        .fold(0, |sum, x| {
            let mut fuelsum = sum;
            let mut fuel = fuel_fuel_req(*x);
            while fuel > 0 {
                fuelsum += fuel;
                fuel = fuel_fuel_req(fuel);
            }
            return fuelsum;
        })
}

#[cfg(test)]
mod tests {
    use super::{part1};

    // examples
    #[test]
    fn sample1() {
        assert_eq!(part1([12]), 2);
        assert_eq!(part1([14]), 2);
        assert_eq!(part1([1969]), 654);
        assert_eq!(part1([100756]), 33583);
        assert_eq!(part1(vec![12, 14, 1969, 100756]), 34241);
    }

    // pt 2 examples
    #[test]
    fn sample2() {
        assert_eq!(part2([12]), 2);
        assert_eq!(part1([14]), 2);
        assert_eq!(part1([1969]), 996);
        assert_eq!(part1([100756]), 50346);
        assert_eq!(part1(vec![12, 14, 1969, 100756]), 51346);
    }
}
