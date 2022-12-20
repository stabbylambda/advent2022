use common::get_raw_input;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use rayon::prelude::*;

fn main() {
    let input = get_raw_input();
    let input = parse(&input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            tuple((
                (delimited(tag("Blueprint "), nom_u32, tag(": "))),
                (delimited(tag("Each ore robot costs "), nom_u32, tag(" ore. "))),
                (delimited(tag("Each clay robot costs "), nom_u32, tag(" ore. "))),
                (tuple((
                    delimited(tag("Each obsidian robot costs "), nom_u32, tag(" ore ")),
                    delimited(tag("and "), nom_u32, tag(" clay. ")),
                ))),
                (tuple((
                    delimited(tag("Each geode robot costs "), nom_u32, tag(" ore ")),
                    delimited(tag("and "), nom_u32, tag(" obsidian.")),
                ))),
            )),
            |(id, ore, clay, (obs_ore, obs_clay), (geode_ore, geode_obs))| {
                Blueprint::new(
                    id,
                    [
                        [ore, 0, 0, 0],
                        [clay, 0, 0, 0],
                        [obs_ore, obs_clay, 0, 0],
                        [geode_ore, 0, geode_obs, 0],
                    ],
                )
            },
        ),
    )(input);

    result.unwrap().1
}

type Input = Vec<Blueprint>;
type Costs = [[u32; 4]; 4];
#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: [[u32; 4]; 4],
    max_costs: [u32; 4],
}

impl Blueprint {
    fn new(id: u32, costs: Costs) -> Blueprint {
        Blueprint {
            id,
            costs,
            max_costs: Blueprint::max_items_needed(&costs),
        }
    }
    fn max_items_needed(costs: &Costs) -> [u32; 4] {
        let mut result = [0, 0, 0, u32::MAX];

        for cost in costs {
            for (item, &amount) in cost.iter().enumerate() {
                result[item] = result[item].max(amount);
            }
        }

        return result;
    }
}

#[derive(Debug, Clone, Copy)]
struct Inventory {
    items: [u32; 4],
    bots: [u32; 4],
    time_left: u32,
}

#[derive(Debug, Clone, Copy)]
enum Types {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl Inventory {
    fn new(time_left: u32) -> Inventory {
        Inventory {
            items: [0, 0, 0, 0],
            bots: [1, 0, 0, 0],
            time_left,
        }
    }

    fn gather(&mut self, times: u32) {
        for n in 0..4 {
            self.items[n] += self.bots[n] * times;
        }
    }

    fn can_build(&self, bp: &Blueprint, robot_type: Types) -> bool {
        let robot_type = robot_type as usize;
        let need_more = self.bots[robot_type] < bp.max_costs[robot_type];

        need_more
            && bp.costs[robot_type]
                .iter()
                .zip(self.items)
                .all(|(&cost, items)| cost <= items)
    }

    fn build_robot(&mut self, bp: &Blueprint, robot_type: Types) {
        self.bots[robot_type as usize] += 1;

        for idx in 0..4 {
            self.items[idx] -= bp.costs[robot_type as usize][idx];
        }
    }
}

fn best_possible(inventory: &Inventory, idx: usize) -> u32 {
    let item_count = inventory.items[idx];
    let bot_collection_count = inventory.bots[idx] * inventory.time_left;
    let bots_to_be_added = inventory.time_left * (inventory.time_left - 1) / 2;

    item_count + bot_collection_count + bots_to_be_added
}

fn simulate(inventory: Inventory, bp: &Blueprint, best_so_far: u32) -> u32 {
    // just stop if there's no more time left, these are the geodes we have plus the ones the bots will collect
    if inventory.time_left == 1 {
        let geodes = inventory.items[Types::Geode as usize] + inventory.bots[Types::Geode as usize];
        return geodes;
    }

    // this branch is trash, just get out of it
    if best_possible(&inventory, Types::Geode as usize) < best_so_far {
        return 0;
    }

    let mut new_inventory = inventory.clone();
    let mut best = best_so_far;

    // gather items and burn time
    new_inventory.time_left -= 1;
    new_inventory.gather(1);

    // we're going to try doing geode first because that's the most important one to find
    for robot_type in [Types::Geode, Types::Ore, Types::Clay, Types::Obsidian] {
        if !inventory.can_build(bp, robot_type) {
            continue;
        }

        let mut branch_with_new_robot = new_inventory.clone();
        branch_with_new_robot.build_robot(bp, robot_type);
        let score = simulate(branch_with_new_robot, bp, best);
        best = score.max(best);
    }

    // I guess...we do nothing?
    let score = simulate(new_inventory, bp, best);
    best = score.max(best);

    best
}

fn get_blueprint_score(bp: &Blueprint) -> u32 {
    let i = Inventory::new(24);
    bp.id * simulate(i, bp, 0)
}

const MINUTES: u32 = 24;
fn problem1(input: &Input) -> u32 {
    input.par_iter().map(get_blueprint_score).sum()
}

fn problem2(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem1(&input);
        assert_eq!(result, 33)
    }

    #[test]
    #[ignore = "reason"]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
