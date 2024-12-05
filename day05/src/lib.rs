use std::cmp::Ordering;

type Rules = Vec<(u32, u32)>;
type Update = Vec<u32>;

fn pair_breaks_rules(rules: &Rules, a: u32, b: u32) -> bool {
    rules.contains(&(b, a))
}

fn update_breaks_rules(rules: &Rules, u: &Update) -> bool {
    let length = u.len();

    for i in 0..length - 1 {
        for k in i + 1..length {
            let (a, b) = (u.get(i).unwrap(), u.get(k).unwrap());

            if pair_breaks_rules(rules, *a, *b) {
                return true;
            }
        }
    }

    false
}

fn correct_sorting(rules: &Rules, u: &Update) -> Update {
    let mut sorted = u.clone();
    sorted.sort_by(|a, b| {
        if pair_breaks_rules(rules, *a, *b) {
            return Ordering::Greater;
        }

        Ordering::Less
    });

    sorted
}

fn parse_input(input: &str) -> (Rules, Vec<Update>) {
    let (o, u) = input.split_once("\n\n").unwrap();

    let rules = o
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("|").unwrap();
            let l = l.parse::<u32>().unwrap();
            let r = r.parse::<u32>().unwrap();

            (l, r)
        })
        .collect::<Vec<_>>();

    let updates = u
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, updates)
}

pub fn part1(input: &str) -> String {
    let (rules, updates) = parse_input(input);

    let result = updates
        .iter()
        .filter_map(|u| {
            (!update_breaks_rules(&rules, u)).then(|| {
                let middle = u.len() / 2;
                u.get(middle).unwrap()
            })
        })
        .sum::<u32>();

    result.to_string()
}

pub fn part2(input: &str) -> String {
    let (rules, updates) = parse_input(input);

    let result = updates
        .iter()
        .filter_map(|u| {
            update_breaks_rules(&rules, u).then(|| {
                let s = correct_sorting(&rules, u);

                let middle = s.len() / 2;
                s.get(middle).unwrap().clone()
            })
        })
        .sum::<u32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "143");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "123");
    }
}
