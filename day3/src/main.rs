use std::collections::HashSet;

use itertools::Itertools;

fn part1() {
    let lines = shared::read_file("input").unwrap();

    let summed: u32 = lines.into_iter().map(|s| {
        let (part1, part2) = s.split_at(s.len() / 2);

        let mut s1 = HashSet::new();
        part1.chars().for_each(|it| {
            s1.insert(it);
        });

        let mut s2 = HashSet::new();
        part2.chars().for_each(|it| {
            s2.insert(it);
        });

        &s1 & &s2
    }).map(|v| {
        let c = v.iter().exactly_one().expect("not one?");

        let asciival = c.clone() as u32;
        if asciival >= 97 {
            asciival - 96
        } else {
            asciival - 38
        }
    }).sum();

    println!("{}", summed)
}

fn part2() {
    let lines = shared::read_file("input").unwrap();

   let summed: u32 = lines.iter().chunks(3).into_iter().
   map(|c| c.collect::<Vec<&String>>()).
   map(|c| {
        let s = c.as_slice();

        // let (part1, part2) = s.split_at(s.len() / 2);

        let mut s1 = HashSet::new();
        s[0].chars().for_each(|it| {
            s1.insert(it);
        });

        let mut s2 = HashSet::new();
        s[1].chars().for_each(|it| {
            s2.insert(it);
        });

        let mut s3 = HashSet::new();
        s[2].chars().for_each(|it| {
            s3.insert(it);
        });

        let ts = (&s1 & &s2);
        &ts & &s3
    }).map(|v| {
        let c = v.iter().exactly_one().expect("not one?");

        let asciival = c.clone() as u32;
        if asciival >= 97 {
            asciival - 96
        } else {
            asciival - 38
        }
    }).sum();

    println!("{}", summed)
}


fn main() {
    part1();
    part2();
}