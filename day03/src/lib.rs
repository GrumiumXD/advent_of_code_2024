use regex::Regex;

pub fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let result = re
        .find_iter(input)
        .map(|m| {
            let s = m.as_str();

            let numbers = s.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();

            let (l, r) = numbers.split_once(",").unwrap();

            let l = l.parse::<u32>().unwrap();
            let r = r.parse::<u32>().unwrap();

            l * r
        })
        .sum::<u32>();

    result.to_string()
}

pub fn part2(input: &str) -> String {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let result = input
        .split("do()")
        .map(|between_dos| {
            let enabled = between_dos.split("don't()").next().unwrap();

            re.find_iter(enabled)
                .map(|m| {
                    let s = m.as_str();

                    let numbers = s.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();

                    let (l, r) = numbers.split_once(",").unwrap();

                    let l = l.parse::<u32>().unwrap();
                    let r = r.parse::<u32>().unwrap();

                    l * r
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const INPUT2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn p1() {
        let result = part1(INPUT1);
        assert_eq!(result, "161");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT2);
        assert_eq!(result, "48");
    }
}
