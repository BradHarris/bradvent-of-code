use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solver::Solver;

#[derive(Default, Debug, Clone)]
struct Valve {
    bit_mask: u32,
    key: String,
    flow_rate: usize,
    neighbors: Vec<(usize, String)>,
}

impl FromStr for Valve {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, rest) = s
            .trim_start_matches("Valve ")
            .split_once(" has flow rate=")
            .unwrap();
        let (flow_rate, neighbors) = rest.split_once(";").unwrap();
        let neighbors = neighbors
            .trim_start_matches(|c: char| c.is_ascii_lowercase() || c.is_ascii_whitespace());
        Ok(Self {
            bit_mask: 0,
            key: key.to_string(),
            flow_rate: flow_rate.parse().unwrap(),
            neighbors: neighbors.split(", ").map(|l| (1, l.to_string())).collect(),
        })
    }
}

#[derive(Default, Debug, Clone)]
struct ValveNetwork(HashMap<String, Valve>);

impl ValveNetwork {
    fn bfs_iter(&self, start: String) -> ValveNetworkBFSIter {
        ValveNetworkBFSIter::new(&self, start)
    }

    fn get_neighbors(&self, target: &str) -> Vec<(usize, Valve)> {
        self.0
            .get(target)
            .unwrap()
            .neighbors
            .iter()
            .map(|(mins, key)| (*mins, self.0.get(key).unwrap().clone()))
            .collect()
    }
}

struct ValveNetworkBFSIter<'a> {
    network: &'a ValveNetwork,
    visited: HashSet<String>,
    to_visit: Vec<(usize, String)>,
}

impl<'a> ValveNetworkBFSIter<'a> {
    fn new(network: &'a ValveNetwork, start: String) -> Self {
        Self {
            network,
            visited: HashSet::new(),
            to_visit: vec![(1, start)],
        }
    }
}

impl<'a> Iterator for ValveNetworkBFSIter<'a> {
    type Item = (usize, &'a Valve);

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_visit.len() == 0 {
            return None;
        }

        let next = self.to_visit.drain(0..1).next()?;

        let valve = self.network.0.get(&next.1).unwrap();
        self.visited.insert(next.1);

        let to_visit = valve
            .neighbors
            .iter()
            .map(|v| (next.0 + v.0, v.1.clone()))
            .filter(|v| {
                let visited = !self.visited.contains(&*v.1);
                let to_visited = !self.to_visit.contains(v);
                visited && to_visited
            })
            .collect::<Vec<(usize, String)>>();

        self.to_visit.extend(to_visit.into_iter());

        Some((next.0, valve))
    }
}

fn find_optimal_flow(
    dp: &mut HashMap<u32, usize>,
    network: &ValveNetwork,
    current_key: String,
    minutes: usize,
    open_valves: u32, // bit mask of open valves
    flowed: usize,
) {
    let current = dp.get(&open_valves).unwrap_or(&0).to_owned();
    dp.insert(open_valves, flowed.max(current));

    network
        .get_neighbors(&current_key)
        .iter()
        .filter(|(mins, v)| *mins < minutes && (v.bit_mask & open_valves) == 0)
        .for_each(|(mins, v)| {
            find_optimal_flow(
                dp,
                network,
                v.key.clone(),
                minutes - mins,
                open_valves | v.bit_mask,
                flowed + (v.flow_rate * (minutes - mins)),
            );
        });
}

#[derive(Default, Debug)]
pub struct Solution {
    input: ValveNetwork,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        let network = ValveNetwork(
            input
                .lines()
                .map(|l| {
                    let valve = l.parse::<Valve>().unwrap();
                    (valve.key.clone(), valve)
                })
                .collect::<HashMap<String, Valve>>(),
        );

        // compact network
        let mut compact_network = ValveNetwork(
            network
                .0
                .iter()
                .filter(|(_, v)| v.flow_rate > 0)
                .enumerate()
                .map(|(i, (key, v))| {
                    // println!("{key}, {}", 2u32.pow(i as u32));
                    (
                        key.clone(),
                        Valve {
                            bit_mask: 2u32.pow(i as u32),
                            key: key.clone(),
                            flow_rate: v.flow_rate,
                            // using network iterator we can build a complete
                            // list of navigatable neighbors ordered by weight
                            neighbors: network
                                .bfs_iter(key.clone())
                                .filter(|v| &v.1.key != key && v.1.flow_rate > 0)
                                .map(|(mins, v)| (mins, v.key.clone()))
                                .collect::<Vec<(usize, String)>>(),
                        },
                    )
                })
                .collect::<HashMap<String, Valve>>(),
        );

        // special case to insert start node with last bit_mask
        let start_key = "AA".to_string();
        compact_network.0.insert(
            start_key.clone(),
            Valve {
                bit_mask: 2u32.pow(compact_network.0.len() as u32),
                key: start_key.clone(),
                flow_rate: 0,
                // using network iterator we can build a complete
                // list of navigatable neighbors ordered by weight
                neighbors: network
                    .bfs_iter(start_key.clone())
                    .filter(|v| &v.1.key != &start_key && v.1.flow_rate > 0)
                    .map(|(mins, v)| (mins, v.key.clone()))
                    .collect::<Vec<(usize, String)>>(),
            },
        );

        self.input = compact_network;
    }

    fn solve_part1(&self) -> String {
        let mut solutions = HashMap::new();
        find_optimal_flow(
            &mut solutions,
            &self.input.clone(),
            "AA".to_string(),
            30,
            0,
            0,
        );

        let max_flow = solutions.iter().max_by_key(|s| s.1).unwrap();

        // println!(
        //     "{} {:?}",
        //     solutions.len(),
        //     self.input
        //         .0
        //         .iter()
        //         .filter(|(_, v)| v.bit_mask & *max_flow.0 != 0)
        //         .map(|(k, _)| k.clone())
        //         .collect::<Vec<String>>()
        // );

        max_flow.1.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut solutions = HashMap::new();
        let valve_count = self.input.0.len() as u32 - 1;
        let bit_total = 2_u32.pow(valve_count as u32) - 1;

        for i in 1..bit_total {
            if i % 10 == 0 {
                println!("{i} / {bit_total} - {}", solutions.len());
            }

            if solutions.contains_key(&i) {
                continue;
            }
            find_optimal_flow(
                &mut solutions,
                &self.input.clone(),
                "AA".to_string(),
                26,
                i,
                0,
            );
        }

        println!("{} {}", valve_count, solutions.len());

        let max_flow = solutions
            .iter()
            .map(|(bit_mask, flow)| {
                let bit_complement = !bit_mask.to_owned() & bit_total;
                let other_flow = solutions.get(&bit_complement).unwrap_or(&0).to_owned();
                println!(
                    "{bit_mask:b} {bit_complement:b} {} {flow} {other_flow}",
                    solutions.contains_key(&bit_complement)
                );
                flow.to_owned() + other_flow
            })
            .max()
            .unwrap();

        max_flow.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_input<'a>() -> &'a str {
        "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
    }

    #[test]
    fn test_parse_example() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "1651");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "1707");
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "2059");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }
}

const INPUT: &str = "\
Valve JI has flow rate=21; tunnels lead to valves WI, XG
Valve DM has flow rate=3; tunnels lead to valves JX, NG, AW, BY, PF
Valve AZ has flow rate=0; tunnels lead to valves FJ, VC
Valve YQ has flow rate=0; tunnels lead to valves TE, OP
Valve WI has flow rate=0; tunnels lead to valves JI, VC
Valve NE has flow rate=0; tunnels lead to valves ZK, AA
Valve FM has flow rate=0; tunnels lead to valves LC, DU
Valve QI has flow rate=0; tunnels lead to valves TE, JW
Valve OY has flow rate=0; tunnels lead to valves XS, VF
Valve XS has flow rate=18; tunnels lead to valves RR, OY, SV, NQ
Valve NU has flow rate=0; tunnels lead to valves IZ, BD
Valve JX has flow rate=0; tunnels lead to valves DM, ZK
Valve WT has flow rate=23; tunnels lead to valves OV, QJ
Valve KM has flow rate=0; tunnels lead to valves TE, OL
Valve NG has flow rate=0; tunnels lead to valves II, DM
Valve FJ has flow rate=0; tunnels lead to valves AZ, II
Valve QR has flow rate=0; tunnels lead to valves ZK, KI
Valve KI has flow rate=9; tunnels lead to valves ZZ, DI, TL, AJ, QR
Valve ON has flow rate=0; tunnels lead to valves LC, QT
Valve AW has flow rate=0; tunnels lead to valves DM, AA
Valve HI has flow rate=0; tunnels lead to valves TE, VC
Valve XG has flow rate=0; tunnels lead to valves II, JI
Valve II has flow rate=19; tunnels lead to valves LF, NG, OL, FJ, XG
Valve VC has flow rate=24; tunnels lead to valves WI, HI, AZ
Valve VJ has flow rate=0; tunnels lead to valves UG, AA
Valve IZ has flow rate=0; tunnels lead to valves VF, NU
Valve EJ has flow rate=0; tunnels lead to valves ZK, LC
Valve DU has flow rate=12; tunnels lead to valves TC, UG, FM
Valve ZK has flow rate=10; tunnels lead to valves JX, EJ, JW, QR, NE
Valve XF has flow rate=25; tunnels lead to valves OP, VT
Valve LC has flow rate=4; tunnels lead to valves FM, EJ, ON, AJ, PF
Valve SV has flow rate=0; tunnels lead to valves XS, IY
Valve LF has flow rate=0; tunnels lead to valves II, OV
Valve DI has flow rate=0; tunnels lead to valves KI, BY
Valve OP has flow rate=0; tunnels lead to valves YQ, XF
Valve NQ has flow rate=0; tunnels lead to valves TC, XS
Valve QJ has flow rate=0; tunnels lead to valves VT, WT
Valve IY has flow rate=22; tunnel leads to valve SV
Valve AJ has flow rate=0; tunnels lead to valves LC, KI
Valve TE has flow rate=11; tunnels lead to valves QI, HI, KM, YQ
Valve ZZ has flow rate=0; tunnels lead to valves KI, AA
Valve VT has flow rate=0; tunnels lead to valves XF, QJ
Valve OL has flow rate=0; tunnels lead to valves KM, II
Valve TC has flow rate=0; tunnels lead to valves NQ, DU
Valve TL has flow rate=0; tunnels lead to valves VF, KI
Valve QT has flow rate=0; tunnels lead to valves AA, ON
Valve BY has flow rate=0; tunnels lead to valves DM, DI
Valve OV has flow rate=0; tunnels lead to valves LF, WT
Valve VN has flow rate=0; tunnels lead to valves RR, BD
Valve VF has flow rate=13; tunnels lead to valves OY, IZ, TL
Valve BD has flow rate=17; tunnels lead to valves NU, VN
Valve UG has flow rate=0; tunnels lead to valves VJ, DU
Valve PF has flow rate=0; tunnels lead to valves LC, DM
Valve RR has flow rate=0; tunnels lead to valves XS, VN
Valve AA has flow rate=0; tunnels lead to valves QT, ZZ, AW, VJ, NE
Valve JW has flow rate=0; tunnels lead to valves ZK, QI";
