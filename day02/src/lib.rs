use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> String {
    let reports = parse_input(input);

    let count = reports
        .iter()
        .filter(|&report| {
            let mut o = Ordering::Equal;

            for (i, (l, r)) in report.iter().zip(report.iter().skip(1)).enumerate() {
                if i == 0 {
                    o = l.cmp(r);
                }

                // check if all numbers are increasing or decreasing
                if o != l.cmp(r) {
                    return false;
                }

                // check if distance is between 1 and 3
                if l.abs_diff(*r) < 1 || l.abs_diff(*r) > 3 {
                    return false;
                }
            }
            true
        })
        .count();

    count.to_string()
}

pub fn part2(input: &str) -> String {
    let reports = parse_input(input);

    let count = reports
        .iter()
        .filter(|&report| {
            // check if any of the sequences missing a specif element are safe
            (0..report.len()).any(|remove| {
                let mut o = Ordering::Equal;

                // create a new iterator that contains all but the "remove" index
                let report_w_skip = report
                    .iter()
                    .enumerate()
                    .filter_map(|(i, el)| (i != remove).then_some(el));

                for (i, (l, r)) in report_w_skip.clone().zip(report_w_skip.skip(1)).enumerate() {
                    if i == 0 {
                        o = l.cmp(r);
                    }

                    // check if all numbers are increasing or decreasing
                    if o != l.cmp(r) {
                        return false;
                    }

                    // check if distance is between 1 and 3
                    if l.abs_diff(*r) < 1 || l.abs_diff(*r) > 3 {
                        return false;
                    }
                }
                true
            })
        })
        .count();

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "4");
    }
}
