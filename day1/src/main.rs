use std::{io::{self, BufRead}, fs::File, path::Path};


fn read_file(path: &Path) -> io::Result<io::Lines<io::BufReader<File>>>{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(res: &Vec<i32>) {
    let mut max = 0;
    let mut runningsum = 0;
    for it in res {
        if *it == -1 {
            if runningsum > max {
                max = runningsum;
            }
            runningsum = 0;

            continue
        }

        runningsum += it;
    }

    println!("{}", max);
}

fn part2(res: &Vec<i32>) {
    let mut max = 0;
    let mut runningsum = 0;

    let mut sums = vec![];
    for it in res {
        if *it == -1 {
            if runningsum > max {
                max = runningsum;
            }
            sums.push(runningsum);
            runningsum = 0;
            continue
        }

        runningsum += it;
    }

    sums.sort_by(|a, b| b.cmp(a));

    println!("{}", sums[0] + sums[1] + sums[2]);
}

fn main() {
    let content = read_file(Path::new("input")).unwrap();
    let res: Vec<i32> = content.map(|item| {
        let it = item.unwrap();
        if it.is_empty() {
            -1
        } else {
            it.parse::<i32>().unwrap()
        }
    }).collect();

    part1(&res);
    println!();
    part2(&res);
}
