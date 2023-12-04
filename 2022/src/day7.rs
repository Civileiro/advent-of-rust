#![allow(dead_code)]

use itertools::Itertools;
use vfs::{MemoryFS, VfsError, VfsPath};

fn generate_vfs(input: &str) -> Result<VfsPath, VfsError> {
    let root: VfsPath = MemoryFS::new().into();

    let mut curr_dir = root.clone();
    for command in input.split('\n') {
        let a = command.split(' ').collect_vec();
        match a.as_slice() {
            ["$", "ls"] => (), // eewee
            ["$", "cd", "/"] => curr_dir = curr_dir.root(),
            ["$", "cd", ".."] => curr_dir = curr_dir.parent().unwrap_or(curr_dir),
            ["$", "cd", dir] => curr_dir = curr_dir.join(dir)?,
            ["dir", dir] => curr_dir.join(dir)?.create_dir()?,
            [size, name] => curr_dir
                .join(name)?
                .create_file()?
                .write_all(size.as_bytes())?,

            [""] => (),
            _ => unimplemented!(),
        }
    }

    Ok(root)
}

fn dir_size(dir: VfsPath) -> usize {
    dir.read_dir()
        .unwrap()
        .filter_map(|f| {
            if f.is_dir().ok()? {
                Some(dir_size(f))
            } else {
                f.read_to_string().ok()?.parse().ok()
            }
        })
        // .fold((0, 0),|(s1, n1), (s2, n2)| (s1 + s2, n1 + n2))
        .sum()
}

fn sum_dirs(dir: VfsPath) -> Result<usize, VfsError> {
    let mut total = 0usize;
    let mut to_explore = vec![dir];

    while let Some(dir) = to_explore.pop() {
        let size = dir_size(dir.clone());
        if size <= 100000 {
            total += size
        }
        dir.read_dir()?
            .filter(|f| f.is_dir().unwrap())
            .for_each(|d| to_explore.push(d));
    }

    Ok(total)
}

pub fn day7_1(input: &str) -> usize {
    let root = generate_vfs(input).unwrap();

    sum_dirs(root).unwrap()
}

fn dir_sizes(dir: VfsPath) -> Vec<usize> {
    let mut dir_sizes = dir
        .read_dir()
        .unwrap()
        .filter(|f| f.is_dir().unwrap())
        .flat_map(dir_sizes)
        .collect_vec();
    dir_sizes.push(dir_size(dir));
    dir_sizes
}

pub fn day7_2(input: &str) -> usize {
    let root = generate_vfs(input).unwrap();

    let used = dir_size(root.clone());
    let needed_space = used + 30000000 - 70000000;

    let mut sizes = dir_sizes(root);
    sizes.sort();
    sizes.into_iter().find(|&n| n > needed_space).unwrap()
}
