#[derive(Default)]
pub struct Submarine {
    horizontal_position: isize,
    depth: isize,
}

impl Submarine {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn use_instructions(&mut self, instructions: Vec<String>) {
        for line in instructions {
            let words = line.split_whitespace().take(2).collect::<Vec<&str>>();

            if let [direction, amount] = words[..] {
                match direction {
                    "forward" => {
                        self.horizontal_position += amount.parse::<isize>().unwrap();
                    }
                    "down" => {
                        self.depth += amount.parse::<isize>().unwrap();
                    }
                    "up" => {
                        self.depth -= amount.parse::<isize>().unwrap();
                    }
                    _ => panic!("Did not receive a valid direction."),
                }
            }
        }
    }

    pub fn product(&self) -> isize {
        self.depth * self.horizontal_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn sample_one() {
        let instructions = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];

        let mut sub = Submarine::new();
        sub.use_instructions(instructions);

        let result = sub.product();

        assert_eq!(result, 150);
    }

    #[test]
    fn solution() {
        let file = fs::read_to_string("src/two/input.txt").unwrap();

        let instructions = file
            .lines()
            .map(|line| line.parse().expect("Could not parse line in file."))
            .collect::<Vec<String>>();

        let mut sub = Submarine::new();
        sub.use_instructions(instructions);

        let result = sub.product();
        println!("{}", result)
    }
}
