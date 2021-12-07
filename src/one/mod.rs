pub fn part_one() -> usize {
    let depth = include_str!("./sample.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|window| if let [a, b] = **window { a < b } else { false })
        .count();

    depth
}

pub fn part_two() -> usize {
    let depth = include_str!("./sample.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(4)
        .filter(|window| {
            // We only need to compare the first value to the last value
            // since both values contain the second and third value
            if let [one, _, _, four] = **window {
                one < four
            } else {
                false
            }
        })
        .count();

    depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_one() {
        assert_eq!(part_one(), 7);
    }

    #[test]
    fn sample_two() {
        assert_eq!(part_two(), 5);
    }
}
