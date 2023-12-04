#![allow(dead_code)]

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Addx(i64),
    Noop,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let command = line.split_whitespace().collect_vec();
            match command.as_slice() {
                ["addx", n] => Instruction::Addx(n.parse().ok().unwrap()),
                ["noop"] => Instruction::Noop,
                _ => panic!(),
            }
        })
        .collect()
}

pub fn day10_1(input: &str) -> i64 {
    let instruction = parse_instructions(input);

    let mut pc: usize = 0;
    let mut i_cycle: usize = 0;
    let mut reg_x: i64 = 1;
    let mut signal_strength: i64 = 0;
    for cycle in 1.. {
        if pc >= instruction.len() {
            break;
        }
        if (cycle + 20) % 40 == 0 {
            signal_strength += cycle * reg_x;
        }
        match instruction[pc] {
            Instruction::Addx(n) => {
                if i_cycle == 1 {
                    i_cycle = 0;
                    pc += 1;
                    reg_x += n;
                } else {
                    i_cycle += 1;
                }
            }
            Instruction::Noop => {
                pc += 1;
            }
        }
    }

    signal_strength
}

pub fn day10_2(input: &str) -> String {
    let instruction = parse_instructions(input);

    let mut pc: usize = 0;
    let mut i_cycle: usize = 0;
    let mut reg_x: i64 = 1;
    let mut crt_pixels = vec![false; 0];
    for cycle in 1.. {
        if pc >= instruction.len() {
            break;
        }
        let crt_pos = (cycle - 1) % 40;
        if ((reg_x - 1)..=(reg_x + 1)).contains(&crt_pos) {
            crt_pixels.push(true);
        } else {
            crt_pixels.push(false);
        }

        match instruction[pc] {
            Instruction::Addx(n) => {
                if i_cycle == 1 {
                    i_cycle = 0;
                    pc += 1;
                    reg_x += n;
                } else {
                    i_cycle += 1;
                }
            }
            Instruction::Noop => {
                pc += 1;
            }
        }
    }
    let lines = crt_pixels.iter().chunks(40);
    let screen = lines
        .into_iter()
        .flat_map(|line| std::iter::once('\n').chain(line.map(|b| if *b { '#' } else { '.' })));
    String::from_iter(screen)
}
