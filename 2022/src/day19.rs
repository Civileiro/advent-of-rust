#![allow(dead_code)]

use arrayvec::ArrayVec;

#[derive(Debug, Default, Clone, Copy)]
struct Resources {
    ores: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
}

impl Resources {
    pub fn add_ores(&self, ores: i32) -> Self {
        Self {
            ores: self.ores + ores,
            ..*self
        }
    }
    pub fn add_clay(&self, clay: i32) -> Self {
        Self {
            clay: self.clay + clay,
            ..*self
        }
    }
    pub fn add_obsidian(&self, obsidian: i32) -> Self {
        Self {
            obsidian: self.obsidian + obsidian,
            ..*self
        }
    }
}

#[derive(Debug)]
struct BlueprintSim<'a, const N: usize> {
    blueprint: &'a Blueprint,
    resources: Resources,
    minute: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

impl<'a, const N: usize> BlueprintSim<'a, N> {
    pub fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            resources: Default::default(),
            minute: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
    fn mine_for(&self, minutes: i32) -> Resources {
        Resources {
            ores: self.resources.ores + self.ore_robots * minutes,
            clay: self.resources.clay + self.clay_robots * minutes,
            obsidian: self.resources.obsidian + self.obsidian_robots * minutes,
            geodes: self.resources.geodes + self.geode_robots * minutes,
        }
    }

    pub fn next_states(&self) -> ArrayVec<Self, 4> {
        let mut next_states = ArrayVec::new();
        if self.minute + 1 >= N as i32 {
            return next_states;
        }
        if self.obsidian_robots > 0 {
            let ore_left_for_geo = self.blueprint.geode_robot_ore_cost - self.resources.ores;
            let time_for_ore = (ore_left_for_geo - 1) / self.ore_robots + 2;
            let obs_left_for_geo =
                self.blueprint.geode_robot_obsidian_cost - self.resources.obsidian;
            let time_for_obs = (obs_left_for_geo - 1) / self.obsidian_robots + 2;
            let time_needed = time_for_ore.max(time_for_obs);
            if ore_left_for_geo <= 0 && obs_left_for_geo <= 0 {
                next_states.push(Self {
                    minute: self.minute + 1,
                    geode_robots: self.geode_robots + 1,
                    resources: self
                        .mine_for(1)
                        .add_ores(-self.blueprint.geode_robot_ore_cost)
                        .add_obsidian(-self.blueprint.geode_robot_obsidian_cost),
                    ..*self
                });
                return next_states;
            } else if time_needed + self.minute < N as i32 {
                next_states.push(Self {
                    minute: self.minute + time_needed,
                    geode_robots: self.geode_robots + 1,
                    resources: self
                        .mine_for(time_needed)
                        .add_ores(-self.blueprint.geode_robot_ore_cost)
                        .add_obsidian(-self.blueprint.geode_robot_obsidian_cost),
                    ..*self
                });
            }
        }
        if self.clay_robots > 0 {
            let ore_left_for_obs = self.blueprint.obisidian_robot_ore_cost - self.resources.ores;
            let time_for_ore = (ore_left_for_obs - 1) / self.ore_robots + 2;
            let clay_left_for_obs = self.blueprint.obisidian_robot_clay_cost - self.resources.clay;
            let time_for_clay = (clay_left_for_obs - 1) / self.clay_robots + 2;
            let time_needed = time_for_ore.max(time_for_clay);
            if ore_left_for_obs <= 0 && clay_left_for_obs <= 0 {
                next_states.push(Self {
                    minute: self.minute + 1,
                    obsidian_robots: self.obsidian_robots + 1,
                    resources: self
                        .mine_for(1)
                        .add_ores(-self.blueprint.obisidian_robot_ore_cost)
                        .add_clay(-self.blueprint.obisidian_robot_clay_cost),
                    ..*self
                });
                return next_states;
            } else if time_needed + self.minute < N as i32 {
                next_states.push(Self {
                    minute: self.minute + time_needed,
                    obsidian_robots: self.obsidian_robots + 1,
                    resources: self
                        .mine_for(time_needed)
                        .add_ores(-self.blueprint.obisidian_robot_ore_cost)
                        .add_clay(-self.blueprint.obisidian_robot_clay_cost),
                    ..*self
                })
            }
        }
        {
            let ore_left_for_clay = self.blueprint.clay_robot_ore_cost - self.resources.ores;
            let time_for_ore = (ore_left_for_clay - 1) / self.ore_robots + 2;
            if ore_left_for_clay <= 0 {
                next_states.push(Self {
                    minute: self.minute + 1,
                    clay_robots: self.clay_robots + 1,
                    resources: self
                        .mine_for(1)
                        .add_ores(-self.blueprint.clay_robot_ore_cost),
                    ..*self
                });
                return next_states;
            } else if time_for_ore + self.minute < N as i32 {
                next_states.push(Self {
                    minute: self.minute + time_for_ore,
                    clay_robots: self.clay_robots + 1,
                    resources: self
                        .mine_for(time_for_ore)
                        .add_ores(-self.blueprint.clay_robot_ore_cost),
                    ..*self
                })
            }
        }
        {
            let ore_left_for_ore = self.blueprint.ore_robot_ore_cost - self.resources.ores;
            let time_for_ore = (ore_left_for_ore - 1) / self.ore_robots + 2;
            if ore_left_for_ore <= 0 {
                next_states.push(Self {
                    minute: self.minute + 1,
                    ore_robots: self.ore_robots + 1,
                    resources: self
                        .mine_for(1)
                        .add_ores(-self.blueprint.ore_robot_ore_cost),
                    ..*self
                })
            } else if time_for_ore + self.minute < N as i32 {
                next_states.push(Self {
                    minute: self.minute + time_for_ore,
                    ore_robots: self.ore_robots + 1,
                    resources: self
                        .mine_for(time_for_ore)
                        .add_ores(-self.blueprint.ore_robot_ore_cost),
                    ..*self
                })
            }
        }

        next_states
    }
    const fn time_left(&self) -> i32 {
        N as i32 - self.minute
    }
    pub const fn geodes_by_end(&self) -> i32 {
        self.resources.geodes + self.geode_robots * self.time_left()
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obisidian_robot_ore_cost: i32,
    obisidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
}

impl Blueprint {
    pub fn simulate_most_geodes<const N: usize>(&self) -> i32 {
        let mut states = vec![BlueprintSim::<N>::new(self)];
        let mut most_geodes = 0;
        let mut best_records = [(0, 0); N];
        while let Some(state) = states.pop() {
            let next_states = state.next_states();
            if next_states.is_empty() && state.geodes_by_end() > most_geodes {
                most_geodes = state.geodes_by_end();
                continue;
            }
            for stt in next_states {
                let (geo_record, robo_record) = best_records[stt.minute as usize];
                if stt.resources.geodes >= geo_record || stt.geode_robots >= robo_record {
                    best_records[stt.minute as usize] = (
                        stt.resources.geodes.max(geo_record),
                        stt.geode_robots.max(robo_record),
                    );
                    states.push(stt);
                }
            }
        }
        most_geodes
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let rest = line.strip_prefix("Blueprint ").unwrap();
            let (_id, rest) = rest.split_once(": Each ore robot costs ").unwrap();
            let (ore_robot_cost, rest) = rest.split_once(" ore. Each clay robot costs ").unwrap();
            let (clay_robot_cost, rest) =
                rest.split_once(" ore. Each obsidian robot costs ").unwrap();
            let (obsidian_robot_ore_cost, rest) = rest.split_once(" ore and ").unwrap();
            let (obsidian_robot_clay_cost, rest) =
                rest.split_once(" clay. Each geode robot costs ").unwrap();
            let (geode_robot_ore_cost, rest) = rest.split_once(" ore and ").unwrap();
            let (geode_robot_obsidian_cost, _) = rest.split_once(" obsidian").unwrap();
            Blueprint {
                ore_robot_ore_cost: ore_robot_cost.parse().unwrap(),
                clay_robot_ore_cost: clay_robot_cost.parse().unwrap(),
                obisidian_robot_ore_cost: obsidian_robot_ore_cost.parse().unwrap(),
                obisidian_robot_clay_cost: obsidian_robot_clay_cost.parse().unwrap(),
                geode_robot_ore_cost: geode_robot_ore_cost.parse().unwrap(),
                geode_robot_obsidian_cost: geode_robot_obsidian_cost.parse().unwrap(),
            }
        })
        .collect()
}

pub fn day19_1(input: &str) -> usize {
    let blueprints = parse_input(input);
    blueprints
        .into_iter()
        .map(|bp| bp.simulate_most_geodes::<24>())
        .enumerate()
        .fold(0, |acc, (i, geo)| acc + (i + 1) * geo as usize)
}

pub fn day19_2(input: &str) -> i32 {
    let blueprints = parse_input(input);
    blueprints
        .into_iter()
        .take(3)
        .map(|bp| bp.simulate_most_geodes::<32>())
        .product()
    // .sum()
}

const _TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";
