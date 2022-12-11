use common::get_raw_input;
use nom::branch::alt;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = get_raw_input();
    let mut input = parse(&input);

    let score = problem1(&mut input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Monkey>;
#[derive(Debug)]
struct Monkey {
    number: u32,
    items: Vec<u32>,
    operation: Operation,
    divisible_by: u32,
    if_true: usize,
    if_false: usize,
    inspected: u32,
}

#[derive(Debug)]
enum OperationValue {
    Constant(u32),
    Old,
}
#[derive(Debug)]
enum Operation {
    Add(OperationValue),
    Mul(OperationValue),
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(
        separated_pair(
            alt((
                nom::character::complete::char('+'),
                nom::character::complete::char('*'),
            )),
            tag(" "),
            alt((
                map(nom_u32, |x| OperationValue::Constant(x)),
                map(tag("old"), |_| OperationValue::Old),
            )),
        ),
        |(op, value)| match op {
            '+' => Operation::Add(value),
            '*' => Operation::Mul(value),
            _x => panic!("couldn't parse operation {_x}"),
        },
    )(input)
}
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            delimited(tag("Monkey "), nom_u32, tag(":\n")),
            delimited(
                tag("  Starting items: "),
                separated_list1(tag(", "), nom_u32),
                newline,
            ),
            delimited(tag("  Operation: new = old "), parse_operation, newline),
            delimited(tag("  Test: divisible by "), nom_u32, newline),
            delimited(tag("    If true: throw to monkey "), nom_u32, newline),
            preceded(tag("    If false: throw to monkey "), nom_u32),
        )),
        |(number, items, operation, divisible_by, if_true, if_false)| Monkey {
            number,
            items,
            operation,
            divisible_by,
            if_true: if_true as usize,
            if_false: if_false as usize,
            inspected: 0,
        },
    )(input)
}
fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(tag("\n\n"), parse_monkey)(input);
    let monkeys = result.unwrap().1;
    monkeys
}

type ThrowTo = (u32, usize);

impl Monkey {
    fn inspect_all(&mut self) -> Vec<ThrowTo> {
        // figure out where all the items are going
        let results: Vec<(u32, usize)> =
            self.items.iter().map(|item| self.inspect(*item)).collect();

        self.inspected += results.len() as u32;

        // clear out this monkey's items
        self.items.clear();

        results
    }
    fn inspect(&self, item: u32) -> ThrowTo {
        println!("  Monkey inspects an item with a worry level of {}.", item);
        let item = match &self.operation {
            Operation::Add(v) => {
                item + match v {
                    OperationValue::Constant(x) => x,
                    OperationValue::Old => &item,
                }
            }
            Operation::Mul(v) => {
                item * match v {
                    OperationValue::Constant(x) => x,
                    OperationValue::Old => &item,
                }
            }
        };
        println!("  Worry level increases to {}.", item);
        let item = item / 3;
        println!("  Monkey is bored. Worry level goes to {}.", item);
        let result = item % self.divisible_by == 0;
        println!(
            "  Worry level is divisible by {}? {}",
            self.divisible_by, result
        );
        let throw_to = if result { self.if_true } else { self.if_false };

        (item, throw_to)
    }
}

fn round(monkeys: &mut Input) {
    for n in 0..monkeys.len() {
        println!("Monkey {}:", n);
        let monkey = monkeys.get_mut(n).unwrap();

        let results = monkey.inspect_all();

        // distribute to the other monkeys
        for (item, throw_to) in results {
            monkeys.get_mut(throw_to).unwrap().items.push(item);
        }
    }
}

fn print_monkeys(monkeys: &Input) {
    for m in monkeys {
        let items: Vec<String> = m.items.iter().map(|x| x.to_string()).collect();
        let items = items.join(", ");

        println!("Monkey {}: {} | {}", m.number, items, m.inspected);
    }
}

fn problem1(monkeys: &mut Input) -> u32 {
    for _n in 0..20 {
        round(monkeys);
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    let monkey_business = monkeys[0].inspected * monkeys[1].inspected;

    monkey_business
}

fn problem2(lines: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = get_raw_input();
        let mut input = parse(&input);
        let result = problem1(&mut input);
        assert_eq!(result, 10605)
    }

    #[test]
    fn second() {
        let input = get_raw_input();
        let input = parse(&input);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
}
