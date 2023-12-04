#![allow(dead_code)]

use itertools::Itertools;
use std::collections::{hash_map::Entry, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Circle {
    pub center: (i64, i64),
    pub radius: usize,
}

impl Circle {
    pub fn corners(&self) -> [(i64, i64); 4] {
        [
            (self.center.0 + self.radius as i64, self.center.1),
            (self.center.0 - self.radius as i64, self.center.1),
            (self.center.0, self.center.1 + self.radius as i64),
            (self.center.0, self.center.1 - self.radius as i64),
        ]
    }

    pub fn distance(&self, other: &Circle) -> usize {
        ((self.center.0 - other.center.0).abs() + (self.center.1 - other.center.1).abs()) as usize
    }
    pub fn intersects(&self, other: &Circle) -> bool {
        self.distance(other) < self.radius + other.radius
    }
    pub fn inside(&self, point: &(i64, i64)) -> bool {
        let distance = ((self.center.0 - point.0).abs() + (self.center.1 - point.1).abs()) as usize;
        // dbg!(distance, self.radius);
        distance < self.radius
    }
}

fn parse_input(input: &str) -> (Vec<Circle>, HashMap<i64, HashSet<i64>>) {
    let parsed = input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Sensor at x=").unwrap();
            let (x, line) = line.split_once(", y=").unwrap();
            let (y, line) = line.split_once(": closest beacon is at x=").unwrap();
            let (rx, ry) = line.split_once(", y=").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let rx: i64 = rx.parse().unwrap();
            let ry: i64 = ry.parse().unwrap();
            (x, y, rx, ry)
        })
        .collect_vec();
    let circles = parsed
        .iter()
        .map(|&(x, y, rx, ry)| Circle {
            center: (x, y),
            radius: ((x - rx).abs() + (y - ry).abs()) as usize,
        })
        .collect();
    let beacons = parsed.into_iter().fold(
        HashMap::<i64, HashSet<i64>>::new(),
        |mut acc, (_, _, rx, ry)| {
            match acc.entry(ry) {
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(rx);
                }
                Entry::Vacant(e) => {
                    let mut set = HashSet::new();
                    set.insert(rx);
                    e.insert(set);
                }
            }
            acc
        },
    );
    (circles, beacons)
}

fn count_beacont_at(sensors: &[Circle], beacons: Option<&HashSet<i64>>, y: i64) -> usize {
    let mut ranges = sensors
        .iter()
        .filter_map(|c| {
            let distance = (c.center.1 - y).abs();
            let range_radius = c.radius as i64 - distance;
            if range_radius < 0 {
                None
            } else {
                Some(((c.center.0 - range_radius), (c.center.0 + range_radius)))
            }
        })
        .sorted_by(|(x1, _), (x2, _)| x1.cmp(x2));

    let (prex, mut prey) = ranges.next().unwrap();
    let mut sum = prey - prex + 1;

    let mut beacons = match beacons {
        Some(b) => b.iter().sorted().collect_vec().into_iter().peekable(),
        None => vec![].into_iter().peekable(),
    };
    if let Some(&&bx) = beacons.peek() {
        if bx <= prey && bx >= prex {
            sum -= 1;
            beacons.next();
        }
    }
    for (x, y) in ranges {
        if y <= prey {
            continue;
        }
        if x <= prey {
            sum += y - prey;
            if let Some(&&bx) = beacons.peek() {
                if bx > prey && bx <= y {
                    sum -= 1;
                    beacons.next();
                }
            }
        } else {
            sum += y - x + 1;
            if let Some(&&bx) = beacons.peek() {
                if bx >= x && bx <= y {
                    sum -= 1;
                    beacons.next();
                }
            }
        }
        prey = y;
    }
    sum as usize
}

pub fn day15_1(input: &str) -> usize {
    // let (input, line) = (_TEST_INPUT.to_owned(), 10);
    let line = 2_000_000;
    let (sensors, beacons) = parse_input(input);
    count_beacont_at(&sensors, beacons.get(&line), line)
}

fn find_empty(sensors: &[Circle], y: i64) -> Option<i64> {
    let mut ranges = sensors
        .iter()
        .filter_map(|c| {
            let distance = (c.center.1 - y).abs();
            let range_radius = c.radius as i64 - distance;
            if range_radius < 0 {
                None
            } else {
                Some(((c.center.0 - range_radius), (c.center.0 + range_radius)))
            }
        })
        .sorted_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .skip_while(|(_, y)| y < &0);

    let (prex, mut prey) = ranges.next().unwrap();

    if prex > 0 {
        return Some(0);
    };

    for (x, y) in ranges {
        if prey < x - 1 {
            return Some(x - 1);
        }
        prey = prey.max(y);
    }
    None
}

fn lines_with_intersections(sensors: &[Circle]) -> Vec<i64> {
    let mut lines = vec![];
    for i1 in 0..sensors.len() {
        let s1 = &sensors[i1];
        for s2 in sensors.iter().skip(i1) {
            let inside_points: Vec<_> = s2
                .corners()
                .into_iter()
                .filter(|point| s1.inside(point))
                .chain(s1.corners().into_iter().filter(|point| s2.inside(point)))
                .collect();
            if inside_points.len() < 2 {
                continue;
            }
            let p1 = inside_points[0];
            let d1 = (p1.0 + p1.1, -p1.0 + p1.1);
            let p2 = inside_points[1];
            let d2 = (p2.0 + p2.1, -p2.0 + p2.1);

            let y1 = (d1.0 + d2.1) / 2;
            let y2 = (d1.1 + d2.0) / 2;

            lines.push(y1);
            lines.push(y2);
        }
    }
    lines
}

fn find_distress(sensors: &[Circle], limit: i64) -> Option<usize> {
    for y in lines_with_intersections(sensors)
        .iter()
        .filter(|&&y| (0..=limit).contains(&y))
    {
        if let Some(x) = find_empty(sensors, *y) {
            return Some(x as usize * 4_000_000 + *y as usize);
        }
    }
    None
}

pub fn day15_2(input: &str) -> usize {
    let limit = 4_000_000;
    let (sensors, _) = parse_input(input);
    find_distress(&sensors, limit).unwrap()
}

const _TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
