use core::panic;
use std::{collections::HashMap, fs};

enum NodeType {
    Dir,
    File,
}

struct Node<'a> {
    size: i32,
    children: HashMap<&'a str, Node<'a>>,
}

enum Command<'a> {
    Ls,
    Cd(&'a str),
}

fn is_command(line: &str) -> bool {
    return line.starts_with('$');
}

fn parse_command(line: &str) -> Command {
    let s = line.split(" ").collect::<Vec<_>>();
    match s.len() {
        2 => Command::Ls,
        3 => Command::Cd(s[2]),
        _ => panic!("Unexpected command"),
    }
}

fn main() {
    let mut tree = Node {
        size: -1,
        children: HashMap::new(),
    };
    let mut cur_node = &mut tree;

    let lines = fs::read_to_string("input")
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut i = 0;
    while i < lines.len() {
        match parse_command(&lines[i]) {
            Command::Ls => {}
            Command::Cd(dirname) => {
                println!("{}", dirname)
            }
        }

        i += 1;
    }
}
