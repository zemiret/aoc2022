use std::fs;

fn part1() {
    fs::read_to_string("input").unwrap().lines().for_each(|line| {
        let linearr = line.as_bytes();
        let (mut i0, mut i1) = (0, 3);

        while i1 < linearr.len() {
            let mut all_different = true;

            'outer: for i in i0..=i1 {
                for j in i0..=i1 {
                    if i != j && linearr[i] == linearr[j] {
                        all_different = false;
                        break 'outer;
                    }
                }
            }

            if all_different {
                println!("{}", i1+1);
                break;
            }

            i0 += 1;
            i1 += 1;
        }
    });
}

fn part2() {
    fs::read_to_string("input").unwrap().lines().for_each(|line| {
        let linearr = line.as_bytes();
        let (mut i0, mut i1) = (0, 13);

        while i1 < linearr.len() {
            let mut all_different = true;

            'outer: for i in i0..=i1 {
                for j in i0..=i1 {
                    if i != j && linearr[i] == linearr[j] {
                        all_different = false;
                        break 'outer;
                    }
                }
            }

            if all_different {
                println!("{}", i1+1);
                break;
            }

            i0 += 1;
            i1 += 1;
        }
    });
}

fn main() {
    part1();
    part2();
}
