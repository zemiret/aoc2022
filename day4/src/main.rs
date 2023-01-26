use shared;

type ParsedLine = ((i32, i32), (i32, i32));

fn one_cotains_other(s1: (i32, i32), s2: (i32, i32)) -> bool {
    s1.0 <= s2.0 && s1.1 >= s2.1 || s2.0 <= s1.0 && s2.1 >= s1.1
}

fn one_overlaps_with_other(s1: (i32, i32), s2: (i32, i32)) -> bool {
    !(s1.1 < s2.0 || s1.0 > s2.1)
}

fn parse_line(line: &String) -> ParsedLine {
    let mut parts = line
        .split(',')
        .map(|r| {
            let mut part_vec = r
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let p2 = part_vec.pop().unwrap();
            let p1 = part_vec.pop().unwrap();
            (p1, p2)
        })
        .collect::<Vec<(i32, i32)>>();
    let p2 = parts.pop().unwrap();
    let p1 = parts.pop().unwrap();

    (p2, p1)
}

fn part1() {
    let lines = shared::read_file("input").unwrap();
    let summed: i32 = lines
        .iter()
        .map(parse_line)
        .map(|p| match one_cotains_other(p.0, p.1) {
            true => 1,
            false => 0,
        })
        .sum();

    println!("{}", summed);
}

fn part2() {
    let lines = shared::read_file("input").unwrap();
    let summed: i32 = lines
        .iter()
        .map(parse_line)
        .map(|p| match one_overlaps_with_other(p.0, p.1) {
            true => 1,
            false => 0,
        })
        .sum();

    println!("{}", summed);
}

fn main() {
    part1();
    part2();
}
