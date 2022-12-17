use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solver::Solver;

#[derive(Default, Debug, Clone)]
struct Valve {
    key: String,
    flow_rate: usize,
    neighbors: Vec<(String, usize)>,
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
            key: key.to_string(),
            flow_rate: flow_rate.parse().unwrap(),
            neighbors: neighbors.split(", ").map(|l| (l.to_string(), 1)).collect(),
        })
    }
}

#[derive(Default, Debug, Clone)]
struct ValveNetwork(HashMap<String, Valve>);

impl ValveNetwork {
    fn iter(&self, start: String) -> ValveNetworkIter {
        ValveNetworkIter::new(&self, start)
    }
}

struct ValveNetworkIter<'a> {
    network: &'a ValveNetwork,
    visited: HashMap<String, usize>,
    to_visit: Vec<(usize, String)>,
}

impl<'a> ValveNetworkIter<'a> {
    fn new(network: &'a ValveNetwork, start: String) -> Self {
        Self {
            network,
            visited: HashMap::new(),
            to_visit: vec![(1, start)],
        }
    }
}

impl<'a> Iterator for ValveNetworkIter<'a> {
    type Item = (usize, &'a Valve);

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_visit.len() == 0 {
            return None;
        }

        let next = self.to_visit.drain(0..1).next()?;

        let valve = self.network.0.get(&next.1).unwrap();
        self.visited.insert(next.1, next.0);

        self.to_visit.extend(
            valve
                .neighbors
                .iter()
                .filter(|v| !self.visited.contains_key(&*v.0))
                .map(|v| (next.0 + v.1, v.0.clone())),
        );

        Some((next.0, valve))
    }
}

fn find_optimal_flow(
    network: &ValveNetwork,
    open_valves: &HashSet<String>,
    minutes: usize,
    flow_rate: usize,
    flowed: usize,
    current_key: String,
) -> usize {
    let next_valves = network
        .iter(current_key)
        .filter(|(mins, v)| *mins < minutes && v.flow_rate > 0 && !open_valves.contains(&v.key))
        .collect::<Vec<(usize, &Valve)>>();

    if next_valves.len() == 0 {
        return flowed + (flow_rate * minutes);
    }

    next_valves
        .iter()
        .map(|(mins, v)| {
            let mut open_valves = open_valves.clone();
            let is_open = open_valves.contains(&v.key);
            let mut flowed = flowed;
            let mut flow_rate = flow_rate;
            if !is_open {
                open_valves.insert(v.key.clone());
                flowed += flow_rate * mins;
                flow_rate += v.flow_rate;
            }
            find_optimal_flow(
                &network,
                &open_valves,
                minutes - mins,
                flow_rate,
                flowed,
                v.key.clone(),
            )
        })
        .max()
        .unwrap()
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
        self.input = ValveNetwork(
            input
                .lines()
                .map(|l| {
                    let valve = l.parse::<Valve>().unwrap();
                    (valve.key.clone(), valve)
                })
                .collect(),
        );
    }

    fn solve_part1(&self) -> String {
        let flowed = find_optimal_flow(
            &self.input.clone(),
            &HashSet::new(),
            30,
            0,
            0,
            "AA".to_string(),
        );

        flowed.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut valves = self
            .input
            .iter("AA".to_string())
            .filter(|(_, v)| v.flow_rate > 0)
            .map(|v| v.1)
            .collect::<Vec<&Valve>>();

        valves.sort_by_key(|v| v.flow_rate);

        let valves = valves
            .iter()
            .map(|v| v.key.clone())
            .collect::<Vec<String>>();

        let mut biggest_flow = 0;
        for i in 1..valves.len() - 1 {
            let mut valves = valves.clone();
            let open_valves = valves.drain(0..i).collect::<HashSet<String>>();

            let flowed1 = find_optimal_flow(
                &self.input.clone(),
                &open_valves,
                26,
                0,
                0,
                "AA".to_string(),
            );

            let flowed2 = find_optimal_flow(
                &self.input.clone(),
                &valves.into_iter().collect(),
                26,
                0,
                0,
                "AA".to_string(),
            );

            biggest_flow = biggest_flow.max(flowed1 + flowed2);
            println!("flowed: {biggest_flow}");
        }

        biggest_flow.to_string()
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
