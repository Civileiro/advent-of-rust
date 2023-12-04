#![allow(dead_code)]

use std::collections::VecDeque;

use itertools::Itertools;

type Item = u64;

#[derive(Debug)]
struct Monkey {
    _name: usize,
    items: VecDeque<Item>,
    worry_divider: Item,
    divisors_product: Option<u64>,
    operation: Op,
    test: Test,
    inspections: usize,
}

impl Monkey {
    pub fn from_input(input: &str, worry_divider: Item) -> Monkey {
        let mut lines = input.lines();
        let _name: usize = {
            let monkey_name = lines.next().unwrap().chars().filter(|c| c.is_ascii_digit());
            String::from_iter(monkey_name).parse().unwrap()
        };
        let items = {
            let items_line = lines.next().unwrap().split_whitespace().skip(1);
            items_line
                .map(|item| item.chars().filter(|c| c.is_ascii_digit()))
                .map(|i| String::from_iter(i))
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect()
        };
        let operation = Op::from_input(lines.next().unwrap());
        let test = Test::from_lines(lines);
        Self {
            _name,
            items,
            worry_divider,
            divisors_product: None,
            operation,
            test,
            inspections: 0,
        }
    }
    pub fn throw(&mut self) -> Option<(usize, Item)> {
        let item = self.items.pop_front()?;
        self.inspections += 1;
        let item = self.operation.apply(item, self.divisors_product);

        let item = item / self.worry_divider;
        let name = self.test.name_to_throw(item);

        Some((name, item))
    }
    pub fn catch(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

#[derive(Debug)]
enum Op {
    Add(Option<Item>, Option<Item>),
    Mul(Option<Item>, Option<Item>),
}

impl Op {
    pub fn from_input(input: &str) -> Op {
        let line = input.split_whitespace().collect_vec();
        match line.as_slice() {
            ["Operation:", "new", "=", n1, op, n2] => {
                let n1 = n1.parse().ok();
                let n2 = n2.parse().ok();
                match *op {
                    "+" => Op::Add(n1, n2),
                    "*" => Op::Mul(n1, n2),
                    _ => panic!("what the heck is {op:?}"),
                }
            }
            _ => panic!("what the heck is {line:?}"),
        }
    }
    pub fn apply(&self, item: Item, divisors_product: Option<u64>) -> Item {
        let res = match self {
            Op::Add(n1, n2) => n1.unwrap_or(item) + n2.unwrap_or(item),
            Op::Mul(n1, n2) => n1.unwrap_or(item) * n2.unwrap_or(item),
        };
        if let Some(n) = divisors_product {
            res % n
        } else {
            res
        }
    }
}

#[derive(Debug)]
struct Test {
    test: Item,
    case_true: usize,
    case_false: usize,
}

impl Test {
    pub fn from_lines(mut lines: std::str::Lines) -> Test {
        let test = {
            let test = lines.next().unwrap().split_whitespace().collect_vec();
            match test.as_slice() {
                ["Test:", "divisible", "by", n] => n.parse().unwrap(),
                _ => panic!("what the heck is {test:?}"),
            }
        };
        let case_true = {
            let case_true = lines.next().unwrap().split_whitespace().collect_vec();
            match case_true.as_slice() {
                ["If", "true:", "throw", "to", "monkey", name] => name.parse().unwrap(),
                _ => panic!("what the heck is {case_true:?}"),
            }
        };
        let case_false = {
            let case_false = lines.next().unwrap().split_whitespace().collect_vec();
            match case_false.as_slice() {
                ["If", "false:", "throw", "to", "monkey", name] => name.parse().unwrap(),
                _ => panic!("what the heck is {case_false:?}"),
            }
        };
        Self {
            test,
            case_true,
            case_false,
        }
    }
    pub fn name_to_throw(&self, item: Item) -> usize {
        if item % self.test == 0 {
            self.case_true
        } else {
            self.case_false
        }
    }
}

fn parse_input_1(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|i| Monkey::from_input(i, 3))
        .collect()
}

pub fn day11_1(input: &str) -> usize {
    let mut monkeys = parse_input_1(input);

    for _ in 0..20 {
        for curr_m in 0..monkeys.len() {
            while let Some((other_m, item)) = monkeys[curr_m].throw() {
                monkeys[other_m].catch(item);
            }
        }
    }
    let mut inspections = monkeys.into_iter().map(|m| m.inspections).collect_vec();
    inspections.sort_unstable();
    inspections.into_iter().rev().take(2).product()
}

fn parse_input_2(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|i| Monkey::from_input(i, 1))
        .collect()
}

pub fn day11_2(input: &str) -> usize {
    let mut monkeys = parse_input_2(input);
    let divisors_product = monkeys.iter().fold(1, |acc, m| acc * m.test.test);
    monkeys
        .iter_mut()
        .for_each(|m| m.divisors_product = Some(divisors_product));

    for _ in 0..10_000 {
        for curr_m in 0..monkeys.len() {
            while let Some((other_m, item)) = monkeys[curr_m].throw() {
                monkeys[other_m].catch(item);
            }
        }
    }
    let mut inspections = monkeys.into_iter().map(|m| m.inspections).collect_vec();
    inspections.sort_unstable();
    inspections.into_iter().rev().take(2).product()
}

const _TEST_INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1
";
