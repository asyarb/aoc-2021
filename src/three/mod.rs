pub fn part_one() -> usize {
    let input = include_str!("./sample.txt");

    let line_length = input.lines().next().unwrap().chars().count() as u32;
    let line_count = input.lines().count() as u32;

    let gamma = vec![0; line_length.try_into().unwrap()]
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
    let epsilon = !gamma & ((1 << line_length) - 1);

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(part_one(), 198)
    }
}
