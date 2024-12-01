fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];

    for l in input.lines() {
        let mut s = l.split_whitespace();

        let first = s.next().unwrap().parse::<i32>().unwrap();
        let second = s.next().unwrap().parse::<i32>().unwrap();

        left.push(first);
        right.push(second);
    }

    (left, right)
}

pub fn part1(input: &str) -> String {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    let sum = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let (left, right) = parse_input(input);

    let sum = left
        .iter()
        .map(|n| {
            let amount = right.iter().filter(|&x| *x == *n).count();

            (amount as i32) * *n
        })
        .sum::<i32>();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "11");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "31");
    }
}
