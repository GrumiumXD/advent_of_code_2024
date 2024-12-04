use std::collections::HashMap;

struct WordPuzzle {
    width: usize,
    height: usize,
    letters: HashMap<(isize, isize), char>,
}

fn parse_input(input: &str) -> WordPuzzle {
    let mut letters = HashMap::new();

    let height = input.lines().count();
    let mut width = 0;

    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            width = line.chars().count();
        }

        for (x, c) in line.chars().enumerate() {
            letters.insert((x as isize, y as isize), c);
        }
    }

    WordPuzzle {
        width,
        height,
        letters,
    }
}

pub fn part1(input: &str) -> String {
    let wp = parse_input(input);
    let mut count = 0;

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for x in 0..wp.width {
        for y in 0..wp.height {
            let x = x as isize;
            let y = y as isize;
            if let Some('X') = wp.letters.get(&(x, y)) {
                for d in dirs {
                    let m = (x + d.0, y + d.1);
                    let a = (x + 2 * d.0, y + 2 * d.1);
                    let s = (x + 3 * d.0, y + 3 * d.1);

                    if let (Some('M'), Some('A'), Some('S')) =
                        (wp.letters.get(&m), wp.letters.get(&a), wp.letters.get(&s))
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count.to_string()
}

pub fn part2(input: &str) -> String {
    let wp = parse_input(input);
    let mut count = 0;

    for x in 0..wp.width {
        for y in 0..wp.height {
            let x = x as isize;
            let y = y as isize;
            if let Some('A') = wp.letters.get(&(x, y)) {
                let ul = (x - 1, y - 1);
                let ur = (x + 1, y - 1);
                let ll = (x - 1, y + 1);
                let lr = (x + 1, y + 1);

                // M M
                //  A
                // S S
                if let (Some('M'), Some('M'), Some('S'), Some('S')) = (
                    wp.letters.get(&ul),
                    wp.letters.get(&ur),
                    wp.letters.get(&ll),
                    wp.letters.get(&lr),
                ) {
                    count += 1;
                }

                // M S
                //  A
                // M S
                if let (Some('M'), Some('M'), Some('S'), Some('S')) = (
                    wp.letters.get(&ul),
                    wp.letters.get(&ll),
                    wp.letters.get(&ur),
                    wp.letters.get(&lr),
                ) {
                    count += 1;
                }

                // S S
                //  A
                // M M
                if let (Some('M'), Some('M'), Some('S'), Some('S')) = (
                    wp.letters.get(&lr),
                    wp.letters.get(&ll),
                    wp.letters.get(&ul),
                    wp.letters.get(&ur),
                ) {
                    count += 1;
                }

                // S M
                //  A
                // S M
                if let (Some('M'), Some('M'), Some('S'), Some('S')) = (
                    wp.letters.get(&ur),
                    wp.letters.get(&lr),
                    wp.letters.get(&ul),
                    wp.letters.get(&ll),
                ) {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn p1() {
        let result = part1(INPUT);
        assert_eq!(result, "18");
    }

    #[test]
    fn p2() {
        let result = part2(INPUT);
        assert_eq!(result, "9");
    }
}
