use regex::Regex;

#[derive(Clone)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub operation: Operation,
    pub test: Test,
}

#[derive(Clone)]
pub enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Clone)]
pub struct Test {
    pub divisor: usize,
    pub if_true: usize,
    pub if_false: usize,
}

impl Monkey {
    pub fn from(input: &[&str]) -> Self {
        let digit_re = Regex::new(r"(\d+)").unwrap();
        let operation_re = Regex::new(r"(?P<operation>[*|+]) (?P<value>old|\d+)").unwrap();

        let matches = operation_re.captures(input[2]).unwrap();
        let operator = matches.name("operation").unwrap().as_str();
        let value = matches.name("value").unwrap().as_str();

        let operation = match operator {
            "+" => Operation::Add(value.parse().unwrap()),
            "*" => {
                if value == "old" {
                    Operation::Square
                } else {
                    Operation::Multiply(value.parse().unwrap())
                }
            }
            _ => panic!(),
        };

        Self {
            items: digit_re
                .captures_iter(input[1])
                .map(|group| group[0].parse().unwrap())
                .collect(),
            operation,
            test: Test {
                divisor: digit_re
                    .captures(input[3])
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
                if_true: digit_re
                    .captures(input[4])
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
                if_false: digit_re
                    .captures(input[5])
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            },
        }
    }
}

impl Operation {
    pub(crate) fn run(&self, x: usize) -> usize {
        match self {
            Operation::Add(value) => x.wrapping_add(*value),
            Operation::Multiply(value) => x.wrapping_mul(*value),
            Operation::Square => x * x,
        }
    }
}

impl Test {
    pub fn run(&self, x: usize) -> usize {
        if x % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}
