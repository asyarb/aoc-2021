#[derive(Default)]
pub struct Submarine {
    horizontal_position: isize,
    depth: isize,
    aim: isize,
}

impl Submarine {
    pub fn new() -> Self {
        let mut sub = Self {
            ..Default::default()
        };

        include_str!("./sample.txt")
            .lines()
            .map(|line| line.split_once(" ").unwrap())
            .for_each(|(dir, amnt)| {
                let amnt = amnt.parse::<isize>().unwrap();

                match dir {
                    "forward" => {
                        sub.horizontal_position += amnt;
                        sub.depth += sub.aim * amnt;
                    }
                    "down" => sub.aim += amnt,
                    "up" => sub.aim -= amnt,
                    _ => unreachable!(),
                }
            });

        sub
    }

    pub fn product(&self) -> isize {
        self.depth * self.horizontal_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_one() {
        let sub = Submarine::new();

        assert_eq!(sub.product(), 900);
    }
}
