use std::collections::HashSet;

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
                    || ((cur_pos_H.0 == cur_pos_T.0
                        || cur_pos_H.1 == cur_pos_T.1)
                            && ((cur_pos_H.0 - cur_pos_T.0).abs()
                                + (cur_pos_H.1 - cur_pos_T.1).abs())
                                >= 2) // uuugllyyyyyyyy
                {
                    cur_pos_T.0 += (cur_pos_H.0 - cur_pos_T.0).signum();
                    cur_pos_T.1 += (cur_pos_H.1 - cur_pos_T.1).signum();
                    visited.insert(cur_pos_T);
                }
            }
        });


    println!("{}", visited.len());
}

fn main() {
    part1();
}
