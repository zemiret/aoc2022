use std::{borrow::{BorrowMut, Borrow}, collections::HashSet};

struct LineInstruction {
    dir_modif: (i32, i32),
    val: u32,
}

fn parse_line(s: &str) -> LineInstruction {
    let parts = s.split(" ").collect::<Vec<&str>>();

    let dir = match parts[0] {
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("unexpected direction parsing"),
    };

    let val = parts[1].parse::<u32>().expect("parse err");

    LineInstruction {
        dir_modif: dir,
        val: val,
    }
}

fn debugprint(posT: (i32, i32), posH: (i32, i32)) {
    let mut arr = vec![vec!['.'; 6]; 5];
    arr[0][0] = 's';
    arr[posT.1 as usize][posT.0 as usize] = 'T';
    arr[posH.1 as usize][posH.0 as usize] = 'H';
    arr.reverse();
    for row in arr {
        for c in row {
            print!("{} ", c);
        }
        println!()
    }
    println!();
}

fn debugprint2(ropenodes: &Vec<(i32, i32)>) {
    let mut arr = vec![vec![".".to_string(); 6]; 5];
    arr[0][0] = "s".to_string();
    for i in (0..ropenodes.len()).rev() {
        let c = i.to_string();
        arr[ropenodes[i].1 as usize][ropenodes[i].0 as usize] = c;
    }
    arr.reverse();
    for row in arr {
        for c in row {
            print!("{} ", c);
        }
        println!()
    }
    println!();
}

fn part1() {
    let mut cur_pos_H = (0, 0);
    let mut cur_pos_T = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    include_str!("../input")
        .lines()
        .map(parse_line)
        .for_each(|inst| {
            for _ in 0..inst.val {
                cur_pos_H.0 += inst.dir_modif.0;
                cur_pos_H.1 += inst.dir_modif.1;

                if (cur_pos_H.0 != cur_pos_T.0
                    && cur_pos_H.1 != cur_pos_T.1
                    && ((cur_pos_H.0 - cur_pos_T.0).abs() + (cur_pos_H.1 - cur_pos_T.1).abs()) > 2)
                    || ((cur_pos_H.0 == cur_pos_T.0 || cur_pos_H.1 == cur_pos_T.1)
                        && ((cur_pos_H.0 - cur_pos_T.0).abs() + (cur_pos_H.1 - cur_pos_T.1).abs())
                            >= 2)
                // uuugllyyyyyyyy
                {
                    cur_pos_T.0 += (cur_pos_H.0 - cur_pos_T.0).signum();
                    cur_pos_T.1 += (cur_pos_H.1 - cur_pos_T.1).signum();
                    visited.insert(cur_pos_T);
                }
            }
        });

    println!("{}", visited.len());
}

fn should_follow(local_head: &(i32, i32), local_tail: &(i32, i32)) -> bool {
    (local_head.0 != local_tail.0
        && local_head.1 != local_tail.1
        && ((local_head.0 - local_tail.0).abs() + (local_head.1 - local_tail.1).abs()) > 2)
        || ((local_head.0 == local_tail.0 || local_head.1 == local_tail.1)
            && ((local_head.0 - local_tail.0).abs() + (local_head.1 - local_tail.1).abs()) >= 2)
    // uuugllyyyyyyyy
}

fn part2() {
    let mut ropenodes = vec![(0, 0); 10]; // 0 is head, 9 is tail
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    include_str!("../input")
        .lines()
        .map(parse_line)
        .for_each(|inst| {
            for _ in 0..inst.val {
                ropenodes[0].0 += inst.dir_modif.0;
                ropenodes[0].1 += inst.dir_modif.1;

                for i in 0..(ropenodes.len()-1) {
                    let local_head = ropenodes[i];
                    let mut local_tail = ropenodes[i+1].borrow_mut();

                    if should_follow(&local_head, &local_tail) {
                        local_tail.0 += (local_head.0 - local_tail.0).signum();
                        local_tail.1 += (local_head.1 - local_tail.1).signum();
                    }
                }

                visited.insert(ropenodes[ropenodes.len()-1]);
            }
        });

    println!("{}", visited.len());
}

fn main() {
    part2();
}
