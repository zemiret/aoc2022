use core::panic;
use std::{cell::RefCell, collections::HashMap, fmt::Display, fs, rc::Rc};

#[derive(Debug, Clone)]
enum NodeType {
    Dir,
    File,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Dir => {
                write!(f, "d")
            }
            NodeType::File => {
                write!(f, "f")
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    size: i32,
    node_type: NodeType,
    name: String,

    children: HashMap<String, Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn print_fs(&self, spacer: String) {
        println!("{}{}", spacer, self);
        self.children
            .values()
            .for_each(|n| n.as_ref().borrow().print_fs(spacer.clone() + "    "));
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}:{})", self.name, self.node_type, self.size)
    }
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

fn parse_single_entry(line: &str) -> (NodeType, i32, String) {
    let s = line.split(" ").collect::<Vec<_>>();
    match s[0] {
        "dir" => (NodeType::Dir, 0, s[1].to_string()),
        _ => (
            NodeType::File,
            s[0].parse::<i32>().unwrap(),
            s[1].to_string(),
        ),
    }
}
/**
 * 1. "execute commands" to buld the tree and calculate the sizes
 * 2. Do the tree walk to get the nodes - "my size is size of my files if I am a dir or my size if I am a file"
 */
fn main() {
    let tree = Rc::new(RefCell::new(Node {
        size: -1,
        children: HashMap::new(),
        node_type: NodeType::Dir,
        parent: None,
        name: "/".to_string(),
    }));
    let mut cur_node: Rc<RefCell<Node>> = Rc::clone(&tree);

    let lines = fs::read_to_string("input")
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut i = 0;
    let lineslen = lines.len();
    while i < lineslen {
        match parse_command(&lines[i]) {
            Command::Ls => {
                i += 1;
                let mut total_size = 0;
                let mut line = &lines[i];
                while !is_command(line) && i < lineslen {
                    let (nt, si, na) = parse_single_entry(line);

                    let n = Node {
                        size: si,
                        children: HashMap::new(),
                        name: na.to_string(),
                        node_type: nt,
                        parent: Some(Rc::clone(&cur_node)),
                    };

                    cur_node
                        .borrow_mut()
                        .children
                        .insert(na, Rc::new(RefCell::new(n)));

                    total_size += si;
                    i += 1;
                    if i == lineslen {
                        break;
                    }
                    line = &lines[i];
                }
                cur_node.borrow_mut().size = total_size;
            }
            Command::Cd(dirname) => match dirname {
                ".." => {
                    let new_cur = match &cur_node.as_ref().borrow().parent {
                        Some(node) => Rc::clone(node),
                        None => panic!("cd out of tree"),
                    };
                    cur_node = new_cur;
                    i += 1;
                }
                _ => {
                    let new_cur = match cur_node.as_ref().borrow().children.get(dirname) {
                        Some(node) => Rc::clone(node),
                        None => panic!("no child with name: {}", dirname),
                    };
                    cur_node = new_cur;
                    i += 1;
                }
            },
        }
    }

    tree.as_ref().borrow().print_fs("".to_string());
    println!();
}
