use std::fs;

fn part1() {
    let lines: Vec<String> = fs::read_to_string("input").unwrap().lines().map(|x| x.to_string()).collect();
    let (stack_lines, move_lines) = lines.split_at(8);
    let move_lines = move_lines.to_vec().split_off(2);


    let mut stacks = build_stacks(&stack_lines.to_vec());

    for line in &move_lines {
        let (quantity, from, to) = parse_inst(line);

        for _ in 0..quantity {
            let it = stacks[from-1].pop().unwrap();
            stacks[to-1].push(it);
        }
    }

    for stack in &stacks {
        let c =  match stack.len() {
            0 => ' ',
            n => stack[n-1]
        };
        print!("{}", c);
    }

    println!();
}

fn part2() {
    let lines: Vec<String> = fs::read_to_string("input").unwrap().lines().map(|x| x.to_string()).collect();
    let (stack_lines, move_lines) = lines.split_at(8);
    let move_lines = move_lines.to_vec().split_off(2);


    let mut stacks = build_stacks(&stack_lines.to_vec());

    for line in &move_lines {
        let (quantity, from, to) = parse_inst(line);
        let stack_from = &mut stacks[from-1];

        let mut top = stack_from.split_off(stack_from.len() - quantity);
        stacks[to-1].append(&mut top);
    }

    for stack in &stacks {
        let c =  match stack.len() {
            0 => ' ',
            n => stack[n-1]
        };
        print!("{}", c);
    }

    println!();
}

const fn get_stack() -> Vec<char> {
    Vec::new()
}

fn build_stacks(stack_lines: &Vec<String>) -> [Vec<char>; 9] {
    const STACK: Vec<char> = get_stack();
    let mut stacks: [Vec<char>; 9] = [STACK; 9];
    stack_lines.iter().rev().for_each(|line| {
        let mut idx = 1;
        let s = line.as_bytes();
        for it in 0..9 {
            let c = s[idx] as char;
            if c != ' ' {
                stacks[it].push(c);
            }
            idx += 4;
        } 
    });
    stacks
}

fn parse_inst(line: &String) -> (usize, usize, usize) {
    let tokens: Vec<&str> = line.split(" ").collect();
    (
        tokens[1].parse::<usize>().unwrap(),
        tokens[3].parse::<usize>().unwrap(),
        tokens[5].parse::<usize>().unwrap(),
    )
}

fn main() {
    part1();
    part2();
}
