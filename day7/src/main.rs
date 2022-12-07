use common::get_raw_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric0, line_ending, not_line_ending, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use petgraph::{
    algo::toposort,
    dot::{Config, Dot},
    graph::NodeIndex,
    Direction::{Incoming, Outgoing},
    Graph,
};
fn main() {
    let lines = get_raw_input();
    let input = parse_lines(&lines);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    // let score = problem2(&lines);
    // println!("problem 2 score: {score}");
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Data<'a> {
    name: &'a str,
    size: u32,
}
impl<'a> Data<'a> {
    fn new(name: &'a str, size: u32) -> Data<'a> {
        Data { name, size }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum Listing<'a> {
    Directory(Data<'a>),
    File(Data<'a>),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Command<'a> {
    GoToRoot,
    GoUp,
    ChangeDir(&'a str),
    List(Vec<Listing<'a>>),
}

fn parse_listing(s: &str) -> IResult<&str, Vec<Listing>> {
    separated_list1(
        line_ending,
        alt((
            map(
                separated_pair(nom_u32, tag(" "), not_line_ending),
                |(size, name)| Listing::File(Data::new(name, size)),
            ),
            preceded(
                tag("dir "),
                map(alphanumeric0, |x| Listing::Directory(Data::new(x, 0))),
            ),
        )),
    )(s)
}

fn parse_command(s: &str) -> IResult<&str, Command> {
    preceded(
        tag("$ "),
        alt((
            preceded(
                tag("cd "),
                map(not_line_ending, |x| match x {
                    ".." => Command::GoUp,
                    "/" => Command::GoToRoot,
                    _ => Command::ChangeDir(x),
                }),
            ),
            preceded(
                tag("ls"),
                map(preceded(line_ending, parse_listing), |listing| {
                    Command::List(listing)
                }),
            ),
        )),
    )(s)
}

fn parse_lines(s: &str) -> Vec<Command> {
    separated_list1(line_ending, parse_command)(s).unwrap().1
}

fn build_filesystem<'a>(commands: &'a [Command]) -> (Graph<Listing<'a>, ()>, NodeIndex) {
    let mut filesystem: Graph<Listing, ()> = Graph::new();
    // gotta add in a slash just so we have the root index for later
    let root = filesystem.add_node(Listing::Directory(Data::new("/", 0)));
    let mut current = root;
    for command in commands {
        match command {
            Command::GoToRoot => current = root,
            Command::GoUp => {
                current = filesystem
                    .neighbors_directed(current, Incoming)
                    .next()
                    .unwrap();
            }
            Command::ChangeDir(dir) => {
                let mut neighbors = filesystem.neighbors_directed(current, Outgoing);
                // find the outgoing directory
                match neighbors
                    .find(|idx| filesystem[*idx] == Listing::Directory(Data { name: dir, size: 0 }))
                {
                    // if it exists, set the current
                    Some(dir_idx) => current = dir_idx,
                    //otherwise create it and set the current
                    None => {
                        let new_idx = filesystem.add_node(Listing::Directory(Data::new(dir, 0)));
                        filesystem.add_edge(current, new_idx, ());
                        current = new_idx;
                    }
                };
            }
            Command::List(children) => {
                for child in children {
                    let mut neighbors = filesystem.neighbors_directed(current, Outgoing);
                    match neighbors.find(|idx| filesystem[*idx] == *child) {
                        Some(dir_idx) => {}
                        None => {
                            let new_idx = filesystem.add_node(*child);
                            filesystem.add_edge(current, new_idx, ());
                        }
                    };
                }
            }
        }
    }
    (filesystem, root)
}

fn calculate_sizes<'a>(filesystem: &mut Graph<Listing<'a>, ()>, root: NodeIndex) {
    filesystem.reverse();

    let t = toposort(&*filesystem, None).unwrap();
    for idx in t {
        if idx == root {
            continue;
        }
        let parent_idx = filesystem.neighbors(idx).next().unwrap();
        let node = filesystem[idx];
        let parent = &mut filesystem[parent_idx];
        let current_size = match node {
            Listing::Directory(data) => data.size,
            Listing::File(data) => data.size,
        };

        match parent {
            Listing::Directory(data) => data.size += current_size,
            _ => panic!(),
        };
    }
}

fn problem1(commands: &[Command]) -> u32 {
    let (mut filesystem, root) = build_filesystem(commands);
    calculate_sizes(&mut filesystem, root);

    let t = toposort(&filesystem, None).unwrap();
    let size = t
        .iter()
        .filter_map(|idx| {
            let node = filesystem[*idx];
            match node {
                Listing::Directory(data) if data.size <= 100_000 => Some(data.size),
                _ => None,
            }
        })
        .sum();

    size
}

fn problem2(lines: &[Command]) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use common::test::get_raw_input;

    use crate::{
        parse_command, parse_lines, parse_listing, problem1, problem2, Command, Data, Listing,
    };
    #[test]
    fn first() {
        let lines = get_raw_input();
        let input = parse_lines(&lines);
        let result = problem1(&input);
        assert_eq!(result, 95437)
    }

    #[test]
    fn second() {
        let lines = get_raw_input();
        let input = parse_lines(&lines);
        let result = problem2(&input);
        assert_eq!(result, 0)
    }
    #[test]
    fn ls() {
        let ls = parse_command(
            r#"$ ls
dir foo
1234 foo.txt"#,
        );
        assert_eq!(
            ls.unwrap().1,
            Command::List(vec![
                Listing::Directory(Data::new("foo", 0)),
                Listing::File(Data::new("foo.txt", 1234)),
            ])
        );
    }

    #[test]
    fn cd() {
        let cd = parse_command("$ cd ..").unwrap().1;
        assert_eq!(cd, Command::GoUp);

        let cd = parse_command("$ cd /").unwrap().1;
        assert_eq!(cd, Command::GoToRoot);

        let cd = parse_command("$ cd foo").unwrap().1;
        assert_eq!(cd, Command::ChangeDir("foo"));
    }

    #[test]
    fn listing() {
        let listing = parse_listing("dir foo").unwrap().1;
        assert_eq!(listing, vec![Listing::Directory(Data::new("foo", 0))]);

        let listing = parse_listing(
            r#"1234 foo.txt
dir foo
5678 bar.txt"#,
        )
        .unwrap()
        .1;
        assert_eq!(
            listing,
            vec![
                Listing::File(Data::new("foo.txt", 1234)),
                Listing::Directory(Data::new("foo", 0)),
                Listing::File(Data::new("bar.txt", 5678))
            ]
        );
    }
}
