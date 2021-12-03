use std::fs;
use std::path::Path;

fn lines_from_file(path: impl AsRef<Path>) -> Vec<usize> {
    let file = fs::read_to_string(path).expect("Failed to open file.");

    file.lines()
        .map(|line| line.parse().expect("Could not parse line in file."))
        .collect()
}

pub fn part_one(path: impl AsRef<Path>) -> isize {
    let lines = lines_from_file(path);
    let mut increments = 0;

    let mut iter = lines.iter().peekable();

    for _ in 0..iter.len() {
        let curr = iter.next();
        let next = iter.peek();

        if let (Some(curr), Some(&next)) = (curr, next) {
            if next > curr {
                increments += 1;
            }
        }
    }

    increments
}

pub fn part_two(path: impl AsRef<Path>) -> isize {
    let lines = lines_from_file(path);
    let mut increments = 0;

    for items in lines.windows(4) {
        if let [one, two, three, four] = items {
            let curr = one + two + three;
            let next = two + three + four;

            if next > curr {
                increments += 1;
            }
        }
    }

    increments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_one() {
        assert_eq!(part_one("src/puzzles/one/sample.txt"), 7);
    }

    #[test]
    fn sample_two() {
        assert_eq!(part_two("src/puzzles/one/sample.txt"), 5);
    }

    #[test]
    fn solution() {
        let count = part_two("src/puzzles/one/input.txt");
        println!("{}", count);
    }
}
