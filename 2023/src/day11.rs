use crate::grid::{AsciiGrid, Grid};

fn sum_galaxy_distances(grid: &impl Grid<Item = u8>, expansion_size: usize) -> usize {
    let expanded_columns: Vec<_> = grid
        .columns()
        .map(|mut column| column.all(|&c| c == b'.'))
        .collect();
    let expanded_lines: Vec<_> = grid
        .lines()
        .map(|mut line| line.all(|&c| c == b'.'))
        .collect();
    let galaxies: Vec<_> = grid
        .coord_iter()
        .filter(|&coord| grid.get_coord(coord) == Some(&b'#'))
        .collect();
    let mut distances = 0;
    for i in 0..(galaxies.len() - 1) {
        let g0 = galaxies[i];
        for g1 in galaxies.iter().skip(i + 1) {
            let x0 = g0.x.min(g1.x);
            let x1 = g0.x.max(g1.x);
            let y0 = g0.y.min(g1.y);
            let y1 = g0.y.max(g1.y);

            let added_lines =
                expanded_lines[y0..=y1].iter().filter(|b| **b).count() * (expansion_size - 1);
            let added_colums =
                expanded_columns[x0..=x1].iter().filter(|b| **b).count() * (expansion_size - 1);
            distances += x1 - x0 + y1 - y0 + added_lines + added_colums;
        }
    }
    distances
}

pub fn day11_1(input: &str) -> Result<usize, ()> {
    let grid = AsciiGrid::from_ascii(input.as_bytes());
    let distances = sum_galaxy_distances(&grid, 2);
    Ok(distances)
}

pub fn day11_2(input: &str) -> Result<usize, ()> {
    let grid = AsciiGrid::from_ascii(input.as_bytes());
    let distances = sum_galaxy_distances(&grid, 1_000_000);
    Ok(distances)
}

#[cfg(test)]
mod tests {
    use crate::grid::AsciiGrid;

    use super::{day11_1, sum_galaxy_distances};

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_day11_1() {
        let res = day11_1(INPUT);
        assert_eq!(res, Ok(374))
    }

    #[test]
    fn test_day11_2() {
        let grid = AsciiGrid::from_ascii(INPUT.as_bytes());
        let distances = sum_galaxy_distances(&grid, 10);
        assert_eq!(distances, 1030);
        let distances = sum_galaxy_distances(&grid, 100);
        assert_eq!(distances, 8410);
    }
}
