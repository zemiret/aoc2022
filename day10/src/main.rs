fn part1() {
    let mut reg = 1;
    let mut pc = 1;
    let mut sum = 0;

    for line in include_str!("../input").lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        // println!("Start of cycle: {}, reg: {}", pc, reg);

        if pc <= 220 && (pc + 20) % 40 == 0 {
            sum += pc * reg;
        }

        match parts.len() {
            1 => {
                // println!("noop");
                pc += 1;
            }
            2 => {
                // println!("{}", line);
                pc += 1;
                // println!("cycle: {}, reg: {}", pc, reg);
                if pc <= 220 && (pc + 20) % 40 == 0 {
                    sum += pc * reg;
                }
                pc += 1;
                let op = parts[1].parse::<i32>().expect("wrong num");
                reg += op;
            }
            _ => panic!("unrecognized instruction"),
        }
    }

    if pc <= 220 && (pc + 20) % 40 == 0 {
        sum += pc * reg;
    }

    println!("{}", sum);
}

fn main() {
    part1();
}
