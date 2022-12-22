use crate::get_input;
use regex::Regex;
use std::{collections::HashMap, io::Result};

const INPUT: &str = "inputs/day_21.txt";
const TEST: &str = "inputs/test.txt";

// Elapsed time: 51968 us
// Memory Used: 2569.874 kb
pub fn find_root() -> Result<isize> {
    let input = get_input(INPUT)?;
    let monkeys = MonkeyMath::from(input);
    let res = monkeys
        .get(&"root".to_string())
        .unwrap()
        .calculate(&monkeys);
    Ok(res.0 as isize)
}

// Elapsed time: 46595 us
// Memory Used: 2569.874 kb
pub fn find_x() -> Result<isize> {
    let input = get_input(INPUT)?;
    let mut monkeys = MonkeyMath::from(input);
    monkeys.get_mut("humn").unwrap().val = Some((0.0, 1.0));
    monkeys.get_mut("root").unwrap().operation = Some("=".to_string());
    let res = monkeys
        .get(&"root".to_string())
        .unwrap()
        .calculate(&monkeys);
    Ok(res.0 as isize)
}

struct MonkeyMath {
    name: String,
    val: Option<(f64, f64)>,
    dependencies: Option<(String, String)>,
    operation: Option<String>,
}

impl MonkeyMath {
    fn new(
        name: String,
        val: Option<(f64, f64)>,
        dependencies: Option<(String, String)>,
        operation: Option<String>,
    ) -> Self {
        MonkeyMath {
            name,
            val,
            dependencies,
            operation,
        }
    }

    fn from(input: String) -> HashMap<String, MonkeyMath> {
        let re = Regex::new(r"\w{4}|\d+|\+|-|\*|/").unwrap();
        input
            .lines()
            .map(|line| {
                let mut matches = re.find_iter(line);
                let name = matches.next().unwrap().as_str().to_string();
                let first = matches.next().unwrap().as_str().to_string();
                let (val, operation, dependencies) = match first.chars().all(char::is_alphabetic) {
                    true => (
                        None,
                        Some(matches.next().unwrap().as_str().to_string()),
                        Some((first, matches.next().unwrap().as_str().to_string())),
                    ),
                    false => (Some((first.parse().unwrap(), 0.0)), None, None),
                };
                (
                    name.clone(),
                    MonkeyMath::new(name, val, dependencies, operation),
                )
            })
            .collect()
    }

    fn calculate(&self, monkeys: &HashMap<String, MonkeyMath>) -> (f64, f64) {
        if let Some(val) = self.val {
            return val;
        }
        let dependencies = self.dependencies.as_ref().unwrap();
        let first = monkeys.get(&dependencies.0).unwrap().calculate(monkeys);
        let second = monkeys.get(&dependencies.1).unwrap().calculate(monkeys);
        let res = match self.operation.as_ref().unwrap().as_str() {
            "+" => (first.0 + second.0, first.1 + second.1),
            "-" => (first.0 - second.0, first.1 - second.1),
            "*" => (first.0 * second.0, first.0 * second.1 + second.0 * first.1),
            "/" => (first.0 / second.0, first.1 / second.0),
            "=" => {
                let val = first.0 - second.0;
                let x = second.1 - first.1;
                if val == 0.0 && x == 0.0 {
                    (0.0, 0.0)
                } else if x == 0.0 {
                    panic!("Bad Equation")
                } else {
                    (val / x, 0.0)
                }
            }
            _ => panic!("Bad Operation"),
        };
        res
    }
}
