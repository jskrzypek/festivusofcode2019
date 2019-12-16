#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> [i32] {
    input.split(',').map(|d| d.parse().unwrap())
}

fn parse_instruction(start: i32) -> dyn Fn(&[i32]) -> [i32] {
    |mut st: [i32]| match *(&st[start .. start + 4]) {
        [1, p, q, o] => { st[o] = st[p] + st[q]; return st; },
        [2, p, q, o] => { st[o] = st[p] * st[q]; return st; },
        _            => st
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(state: &[i32]) -> [i32] {
    (0 .. state.len())
        .step_by(4)
        .iter()
        .map(parse_instruction)
        .fold(state, |st, op| op(st))
}

// #[aoc(day2, part1, for_loop)]
// pub fn solve_part1_for(input: &[Gift]) -> u32 {
//     let mut sum = 0;

//     for &(l, w, h) in input {
//         let (s1, s2) = smallest_side((l, w, h));

//         sum += 2 * l * w + 2 * w * h + 2 * h * l + s1 * s2;
//     }

//     sum
// }

// #[aoc(day2, part2)]
// pub fn solve_part2(input: &[Gift]) -> u32 {
//     input
//         .iter()
//         .map(|&(l, w, h)| {
//             let (s1, s2) = smallest_side((l, w, h));

//             (s1 + s2) * 2 + l * w * h
//         }).sum()
// }

// fn smallest_side((l, w, h): Gift) -> (u32, u32) {
//     let mut vec = vec![l, w, h];
//     vec.sort();

//     (vec[0], vec[1])
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    fn example1() {
        assert_eq!(solve_part1(input_generator("1,0,0,0,99")), [2,0,0,0,99]);
    }
    fn example2() {
        assert_eq!(solve_part1(input_generator("2,3,0,3,99")), [2,3,0,6,99]);
    }
    fn example3() {
        assert_eq!(solve_part1(input_generator("2,4,4,5,99,0")), [2,4,4,5,99,9801]);
    }
    fn example4() {
        assert_eq!(solve_part1(input_generator("1,1,1,4,99,5,6,0,99")), [30,1,1,4,2,5,6,0,99]);
    }
}
