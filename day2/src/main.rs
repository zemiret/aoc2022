use std::fs;

enum Result {
    Win,
    Lose,
    Draw,
}

fn get_result(line: Vec<&str>) -> Result {
    match line[0] {
        "A" => match line[1] {
            "X" => Result::Draw,
            "Y" => Result::Win,
            "Z" => Result::Lose,
            _ => panic!("wtf"),
        },
        "B" => match line[1] {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("wtf"),
        },
        "C" => match line[1] {
            "X" => Result::Win,
            "Y" => Result::Lose,
            "Z" => Result::Draw,
            _ => panic!("wtf"),
        },
        _ => panic!("wtf"),
    }
}

fn part1() {
    let res: i32 = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|x| x.split(" ").collect())
        .map(|game: Vec<&str>| {
            let shape_score = match game[1] {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("wtf"),
            };
            let game_score = match get_result(game) {
                Result::Lose => 0,
                Result::Draw => 3,
                Result::Win => 6,
            };

            shape_score + game_score
        }).sum();

    println!("{}", res)
}

fn part2() {
    let res: i32 = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|x| x.split(" ").collect())
        .map(|game: Vec<&str>| {
            let game_score = match game[1] {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!("wtf"),
            };

            let shape_score = match game[0] {
                "A" => match game[1] {
                    "X" => 3,
                    "Y" => 1,
                    "Z" => 2,
                    _ => panic!("wtf"),
                },
                "B" => match game[1] {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => panic!("wtf"),
                },
                "C" => match game[1] {
                    "X" => 2, 
                    "Y" => 3, 
                    "Z" => 1, 
                    _ => panic!("wtf"),
                },
                _ => panic!("wtf"),
            };

            shape_score + game_score
        }).sum();

    println!("{}", res)
}

fn main() {
    part1();
    part2();
}
