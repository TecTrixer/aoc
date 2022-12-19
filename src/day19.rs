use std::collections::HashMap;

use cp_rs::io::*;
#[allow(unused_mut)]
fn main() {
    let mut io = Io::from_file("day19.txt");
    // let mut io = Io::from_file("test.txt");
    let mut blueprints = vec![];
    for line in io.nums::<usize>().chunks(7) {
        let id = line[0];
        let ore_cost = line[1];
        let clay_cost = line[2];
        let obsidian_cost_ore = line[3];
        let obsidian_cost_clay = line[4];
        let geode_cost_ore = line[5];
        let geode_cost_obsidian = line[6];
        blueprints.push(Blueprint {
            id,
            ore_cost,
            clay_cost,
            obsidian_cost_ore,
            obsidian_cost_clay,
            geode_cost_ore,
            geode_cost_obsidian,
            store: HashMap::new(),
        });
    }
    let mut handles = vec![];
    for (i, mut bp) in blueprints.into_iter().enumerate() {
        if i >= 3 {
            break;
        }
        let handle = std::thread::spawn(move || {
            (
                bp.find_max_geodes(State {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                    iterations: 32,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                }),
                bp.id,
            )
        });
        handles.push(handle);
    }
    let mut ress = vec![];
    for handle in handles {
        let (val, id) = handle.join().unwrap();
        io.write(id);
        io.write(" - ");
        io.writeln(val);
        ress.push(val * id);
    }
    let res: usize = ress.iter().product();
    io.write("Part 1: ");
    io.writeln(res);
    io.write("Part 2: ");
    io.writeln(res);
}

struct Blueprint {
    id: usize,
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost_ore: usize,
    obsidian_cost_clay: usize,
    geode_cost_ore: usize,
    geode_cost_obsidian: usize,
    store: HashMap<State, usize>,
}

impl Blueprint {
    fn find_max_geodes(&mut self, state: State) -> usize {
        if state.iterations == 0 {
            return state.geode;
        }
        if let Some(max) = self.store.get(&state) {
            return *max;
        }
        let mut max = 0;
        // buying an ore robot
        if state.iterations >= 15 && state.ore >= self.ore_cost {
            let val = self.find_max_geodes(State {
                ore: state.ore - self.ore_cost + state.ore_robots,
                clay: state.clay + state.clay_robots,
                obsidian: state.obsidian + state.obsidian_robots,
                geode: state.geode + state.geode_robots,
                iterations: state.iterations - 1,
                ore_robots: state.ore_robots + 1,
                clay_robots: state.clay_robots,
                obsidian_robots: state.obsidian_robots,
                geode_robots: state.geode_robots,
            });
            if val > max {
                max = val;
            }
        }
        // buying a clay robot
        if state.iterations >= 10 && state.ore >= self.clay_cost {
            let val = self.find_max_geodes(State {
                ore: state.ore - self.clay_cost + state.ore_robots,
                clay: state.clay + state.clay_robots,
                obsidian: state.obsidian + state.obsidian_robots,
                geode: state.geode + state.geode_robots,
                iterations: state.iterations - 1,
                ore_robots: state.ore_robots,
                clay_robots: state.clay_robots + 1,
                obsidian_robots: state.obsidian_robots,
                geode_robots: state.geode_robots,
            });
            if val > max {
                max = val;
            }
        }
        // buying an obsidian robot
        if state.iterations >= 4 && state.ore >= self.obsidian_cost_ore && state.clay >= self.obsidian_cost_clay {
            let val = self.find_max_geodes(State {
                ore: state.ore - self.obsidian_cost_ore + state.ore_robots,
                clay: state.clay - self.obsidian_cost_clay + state.clay_robots,
                obsidian: state.obsidian + state.obsidian_robots,
                geode: state.geode + state.geode_robots,
                iterations: state.iterations - 1,
                ore_robots: state.ore_robots,
                clay_robots: state.clay_robots,
                obsidian_robots: state.obsidian_robots + 1,
                geode_robots: state.geode_robots,
            });
            if val > max {
                max = val;
            }
        }
        // buying an geode robot
        if state.ore >= self.geode_cost_ore && state.obsidian >= self.geode_cost_obsidian {
            let val = self.find_max_geodes(State {
                ore: state.ore - self.geode_cost_ore + state.ore_robots,
                clay: state.clay + state.clay_robots,
                obsidian: state.obsidian - self.geode_cost_obsidian + state.obsidian_robots,
                geode: state.geode + state.geode_robots,
                iterations: state.iterations - 1,
                ore_robots: state.ore_robots,
                clay_robots: state.clay_robots,
                obsidian_robots: state.obsidian_robots,
                geode_robots: state.geode_robots + 1,
            });
            if val > max {
                max = val;
            }
        }
        // waiting
        let val = self.find_max_geodes(State {
            ore: state.ore + state.ore_robots,
            clay: state.clay + state.clay_robots,
            obsidian: state.obsidian + state.obsidian_robots,
            geode: state.geode + state.geode_robots,
            iterations: state.iterations - 1,
            ore_robots: state.ore_robots,
            clay_robots: state.clay_robots,
            obsidian_robots: state.obsidian_robots,
            geode_robots: state.geode_robots,
        });
        if val > max {
            max = val;
        }
        self.store.insert(state, max);
        max
    }
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    iterations: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}
