#![allow(dead_code)]

use std::cell::RefCell;

use transpose::transpose_inplace;

use itertools::Itertools;

fn parse_crates(crates: &str) -> Vec<RefCell<Vec<char>>> {
    let mut crate_matrix = crates.chars().filter(|c| c != &'\n').collect_vec();
    let columns = {
        let width = crates.chars().position(|c| c == '\n').unwrap();
        let height = crates.chars().filter(|c| c == &'\n').count() + 1;
        let mut scratch = vec!['\0'; std::cmp::max(width, height)];
        transpose_inplace(&mut crate_matrix, &mut scratch, width, height);
        crate_matrix.chunks(height)
    };
    let parsed_crates = columns
        .filter(|&column| column.iter().last() != Some(&' '))
        .map(|column| column.iter().filter(|&c| c != &' ').copied().collect_vec());

    let mut hmap = Vec::new();

    for mut line in parsed_crates {
        let _key = line.pop();
        line.reverse();
        hmap.push(RefCell::new(line));
    }
    hmap
}

pub fn day5_1(input: &str) -> String {
    let (crates, commands) = input.split_once("\n\n").unwrap();

    let crates = parse_crates(crates);

    for command in commands.split('\n') {
        match command.split_whitespace().collect_vec().as_slice() {
            ["move", qnt, "from", from_key, "to", to_key] => {
                let qnt = qnt.parse::<i32>().unwrap();
                let from_key = from_key.chars().next().unwrap() as u8 - b'1';
                let to_key = to_key.chars().next().unwrap() as u8 - b'1';

                for _ in 0..qnt {
                    let elem = crates[from_key as usize].borrow_mut().pop().unwrap();
                    crates[to_key as usize].borrow_mut().push(elem);
                }
            }
            [] => (),
            _ => unimplemented!(),
        }
    }

    let chars = crates.iter().filter_map(|c| c.borrow().last().copied());

    String::from_iter(chars)
}

pub fn day5_2(input: &str) -> String {
    let (crates, commands) = input.split_once("\n\n").unwrap();

    let crates = parse_crates(crates);

    for command in commands.split('\n') {
        match command.split_whitespace().collect_vec().as_slice() {
            ["move", qnt, "from", from_key, "to", to_key] => {
                let qnt = qnt.parse::<usize>().unwrap();
                let from_key = from_key.chars().next().unwrap() as u8 - b'1';
                let to_key = to_key.chars().next().unwrap() as u8 - b'1';

                let stack = &mut crates[from_key as usize].borrow_mut();
                let len = stack.len();
                let elems = stack.drain((len - qnt)..).collect_vec();
                crates[to_key as usize].borrow_mut().extend(elems);
            }
            [] => (),
            _ => unimplemented!(),
        }
    }

    let chars = crates.iter().filter_map(|c| c.borrow().last().copied());

    String::from_iter(chars)
}
