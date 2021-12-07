const BITS: usize = 5;

pub fn part_one() -> usize {
    let input = include_str!("./sample.txt");

    let line_count = input.lines().count() as u32;

    let gamma = vec![0; BITS]
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            input.lines().fold(0 as u32, |acc, line| {
                let bit = line.chars().nth(idx).unwrap().to_digit(2).unwrap();

                acc + bit
            })
        })
        .map(|count| (count >= line_count / 2) as u32)
        .map(|val| val.to_string())
        .collect::<Vec<String>>();

    let gamma = usize::from_str_radix(&gamma.join(""), 2).unwrap();
    let epsilon = !gamma & ((1 << BITS) - 1);

    gamma * epsilon
}

pub fn part_two() -> usize {
    let numbers = include_str!("./sample.txt")
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<usize>>();

    let oxygen = (0..BITS)
        .rev()
        .scan(numbers.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            oxy.drain_filter(|n| (*n & 1 << i > 0) != one);

            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..BITS)
        .rev()
        .scan(numbers, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);

            co2.first().copied()
        })
        .last()
        .unwrap();

    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(part_one(), 198);
        assert_eq!(part_two(), 230);
    }
}
