#![allow(dead_code)]

use smallstr::SmallString;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn from_str(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!("Op cant convert {s:?}"),
        }
    }
    pub fn apply(&self, n1: i64, n2: i64) -> i64 {
        match self {
            Op::Add => n1 + n2,
            Op::Sub => n1 - n2,
            Op::Mul => n1 * n2,
            Op::Div => n1 / n2,
        }
    }
}

type MonkeyName = SmallString<[u8; 4]>;

#[derive(Debug, Clone)]
enum Monkey {
    Num(i64),
    Human(i64),
    Math(MonkeyName, Op, MonkeyName),
}

fn parse_input(input: &str) -> HashMap<MonkeyName, Monkey> {
    input
        .lines()
        .map(|l| {
            let (key, rest) = l.split_once(": ").unwrap();
            let key = MonkeyName::from_str(key);
            if key == "humn" {
                (key, Monkey::Human(rest.parse().unwrap()))
            } else if rest.chars().next().unwrap().is_ascii_digit() {
                (key, Monkey::Num(rest.parse().unwrap()))
            } else {
                let mut math = rest.split_whitespace();
                let key1 = MonkeyName::from_str(math.next().unwrap());
                let op = math.next().unwrap();
                let op = Op::from_str(op);
                let key2 = MonkeyName::from_str(math.next().unwrap());
                (key, Monkey::Math(key1, op, key2))
            }
        })
        .collect()
}

fn get_monkey_num(name: &MonkeyName, monkeys: &HashMap<MonkeyName, Monkey>) -> i64 {
    match &monkeys[name] {
        Monkey::Num(n) => *n,
        Monkey::Human(n) => *n,
        Monkey::Math(k1, op, k2) => {
            op.apply(get_monkey_num(k1, monkeys), get_monkey_num(k2, monkeys))
        }
    }
}

pub fn day21_1(input: &str) -> i64 {
    let monkeys = parse_input(input);
    get_monkey_num(&MonkeyName::from_str("root"), &monkeys)
}

fn get_monkey_num_opt_cache(
    name: MonkeyName,
    monkeys: &mut HashMap<MonkeyName, Monkey>,
) -> Option<i64> {
    match monkeys[&name].clone() {
        Monkey::Num(n) => Some(n),
        Monkey::Human(_) => None,
        Monkey::Math(k1, op, k2) => {
            let left = get_monkey_num_opt_cache(k1, monkeys);
            let right = get_monkey_num_opt_cache(k2, monkeys);
            match (left, right) {
                (Some(n1), Some(n2)) => {
                    let res = op.apply(n1, n2);
                    *monkeys.get_mut(&name).unwrap() = Monkey::Num(res);
                    Some(res)
                }
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (None, None) => unimplemented!(),
            }
        }
    }
}

fn get_monkey_num_opt(name: MonkeyName, monkeys: &HashMap<MonkeyName, Monkey>) -> Option<i64> {
    match monkeys[&name].clone() {
        Monkey::Num(n) => Some(n),
        Monkey::Human(_) => None,
        Monkey::Math(k1, op, k2) => {
            let left = get_monkey_num_opt(k1, monkeys);
            let right = get_monkey_num_opt(k2, monkeys);
            match (left, right) {
                (Some(n1), Some(n2)) => Some(op.apply(n1, n2)),
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (None, None) => unimplemented!(),
            }
        }
    }
}

fn human_should_be(num: i64, curr: &MonkeyName, monkeys: &HashMap<MonkeyName, Monkey>) -> i64 {
    match &monkeys[curr] {
        Monkey::Num(_) => unreachable!(),
        Monkey::Human(_) => num,
        Monkey::Math(left, op, right) => {
            let left_num = get_monkey_num_opt(left.clone(), monkeys);
            let right_num = get_monkey_num_opt(right.clone(), monkeys);
            match (left_num, right_num, op) {
                (None, Some(n), Op::Add) => human_should_be(num - n, left, monkeys),
                (Some(n), None, Op::Add) => human_should_be(num - n, right, monkeys),
                (None, Some(n), Op::Sub) => human_should_be(num + n, left, monkeys),
                (Some(n), None, Op::Sub) => human_should_be(n - num, right, monkeys),
                (None, Some(n), Op::Mul) => human_should_be(num / n, left, monkeys),
                (Some(n), None, Op::Mul) => human_should_be(num / n, right, monkeys),
                (None, Some(n), Op::Div) => human_should_be(num * n, left, monkeys),
                (Some(n), None, Op::Div) => human_should_be(n / num, right, monkeys),
                (None, None, _) => unimplemented!(),
                (Some(_), Some(_), _) => unimplemented!(),
            }
        }
    }
}

pub fn day21_2(input: &str) -> i64 {
    let mut monkeys = parse_input(input);
    let Monkey::Math(left, _, right) = monkeys[&MonkeyName::from_str("root")].clone() else {
        unimplemented!()
    };
    let left_num = get_monkey_num_opt_cache(left.clone(), &mut monkeys);
    let right_num = get_monkey_num_opt_cache(right.clone(), &mut monkeys);
    match (left_num, right_num) {
        (None, Some(n)) => human_should_be(n, &left, &monkeys),
        (Some(n), None) => human_should_be(n, &right, &monkeys),
        (None, None) => unimplemented!(),
        (Some(_), Some(_)) => unimplemented!(),
    }
}

const _TEST_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

const _TEST_INPUT2: &str = "root: juli + josi
juli: amee + alex
amee: buki * abby
buki: 5
abby: 4
alex: 4
josi: benj / mark
benj: 360
mark: emly - humn
emly: 34
humn: 0";
