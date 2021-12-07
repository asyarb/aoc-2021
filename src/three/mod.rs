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
        .collect::<Vec<u32>>();

    let epsilon = gamma
        .iter()
        .map(|bit| if *bit == 1 { 0 } else { 1 })
        .collect::<Vec<u32>>();

    let gamma = gamma
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<String>>();
    let epsilon = epsilon
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<String>>();

    let gamma = usize::from_str_radix(&gamma.join(""), 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon.join(""), 2).unwrap();

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
