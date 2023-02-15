
#[derive(Clone, Default)]
struct HighestSeenYet {
    top: u32,
    bot: u32,
    right: u32,
    left: u32,
}

/**
 * First idea that came to mind;
 * - create an auxiliary gird with the same dimensions holding structs type
 * HighestSeenYet { top: u8, right: u8, bottom: u8, left: u8 }
 * - iterate rows/cols in directions: from left, from top, from right, from bottom and fill our
 * auxilary grid with the proper info (basically higghest_seen_yet at i is either grid[i] or
 * highest_seen[i-1]
 * - iterate auxiliary grid and i_am_visible is basically my_height > any(highest_left, highest_right, highest_bot, highest_top)
 * BUT actually this last step is not correct actually
 * **/
fn part1() {
    let tree_map: Vec<Vec<u32>> = include_str!("../input")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).expect("NaN"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut highest_yet: Vec<Vec<HighestSeenYet>> = tree_map
        .iter()
        .map(|row| {
            vec![
                HighestSeenYet {
                    top: 0,
                    bot: 0,
                    right: 0,
                    left: 0
                };
                row.len()
            ]
        })
        .collect();

    let forest_size = tree_map.len();
    // fill the brim
    for i in 0..forest_size {
        highest_yet[0][i].top = tree_map[0][i];
        highest_yet[0][i].bot = tree_map[0][i];
        highest_yet[0][i].left = tree_map[0][i];
        highest_yet[0][i].right = tree_map[0][i];

        highest_yet[i][0].top = tree_map[i][0];
        highest_yet[i][0].bot = tree_map[i][0];
        highest_yet[i][0].left = tree_map[i][0];
        highest_yet[i][0].right = tree_map[i][0];

        highest_yet[forest_size - 1][i].top = tree_map[forest_size - 1][i];
        highest_yet[forest_size - 1][i].bot = tree_map[forest_size - 1][i];
        highest_yet[forest_size - 1][i].left = tree_map[forest_size - 1][i];
        highest_yet[forest_size - 1][i].right = tree_map[forest_size - 1][i];

        highest_yet[i][forest_size - 1].top = tree_map[i][forest_size - 1];
        highest_yet[i][forest_size - 1].bot = tree_map[i][forest_size - 1];
        highest_yet[i][forest_size - 1].left = tree_map[i][forest_size - 1];
        highest_yet[i][forest_size - 1].right = tree_map[i][forest_size - 1];
    }

    // TODO: Maybe if we are writing in tree_map.at(sth) here we can count that as visible=true and sum that? (also the brim is visible)
    // Ah yeah, counting the brim twice in some cases probably, hmm...

    for i in 0..forest_size {
        for j in 1..forest_size {
            // rows iter left to right
            highest_yet[i][j].left = if tree_map[i][j] > highest_yet[i][j - 1].left {
                tree_map[i][j]
            } else {
                highest_yet[i][j - 1].left
            };
            // rows iter right to left
            highest_yet[i][forest_size - j - 1].right =
                if tree_map[i][forest_size - j - 1] > highest_yet[i][forest_size - j].right {
                    tree_map[i][forest_size - j - 1]
                } else {
                    highest_yet[i][forest_size - j].right
                };
            // rows iter top to bot
            highest_yet[j][i].top = if tree_map[j][i] > highest_yet[j - 1][i].top {
                tree_map[j][i]
            } else {
                highest_yet[j - 1][i].top
            };
            // rows iter bot to top
            highest_yet[forest_size - j - 1][i].bot =
                if tree_map[forest_size - j - 1][i] > highest_yet[forest_size - j][i].bot {
                    tree_map[forest_size - j - 1][i]
                } else {
                    highest_yet[forest_size - j][i].bot
                };
        }
    }

    let mut visibility_map: Vec<Vec<u8>> = tree_map.iter().map(|row| vec![0; row.len()]).collect();
    for i in 0..forest_size {
        visibility_map[i][0] = 1;
        visibility_map[0][i] = 1;
        visibility_map[forest_size-1][i] = 1;
        visibility_map[i][forest_size-1] = 1;
    }

    let mut total_visible = 4 * forest_size - 4;
    for i in 1..forest_size - 1 {
        for j in 1..forest_size - 1 {
            if tree_map[i][j] > highest_yet[i - 1][j].top
                || tree_map[i][j] > highest_yet[i + 1][j].bot
                || tree_map[i][j] > highest_yet[i][j - 1].left
                || tree_map[i][j] > highest_yet[i][j + 1].right
            {
                visibility_map[i][j] = 1;
                total_visible += 1;
            }
        }
    }

    println!("Input: ");
    for i in 0..forest_size {
        for j in 0..forest_size {
            print!("{}", tree_map[i][j]);
        }
        println!();
    }
    println!();

    println!("Visibility map: ");
    for i in 0..forest_size {
        for j in 0..forest_size {
            print!("{}", visibility_map[i][j]);
        }
        println!();
    }
    println!();

    println!("From left:");
    for i in 0..forest_size {
        for j in 0..forest_size {
            print!("{}", highest_yet[i][j].left);
        }
        println!();
    }
    println!();

    println!("From right:");
    for i in 0..forest_size {
        for j in 0..forest_size {
            print!("{}", highest_yet[i][j].right);
        }
        println!();
    }
    println!();

    println!("From top:");
    for i in 0..forest_size {
        for j in 0..forest_size {
            print!("{}", highest_yet[i][j].top);
        }
        println!();
    }
    println!();

    println!("From bot:");
    for i in 0..forest_size {
        for j in 0..forest_size {
            print!("{}", highest_yet[i][j].bot);
        }
        println!();
    }
    println!();
    println!("{}", total_visible);
}

/**
 * Given the size of input data we can just brute-force this.
 * A dynamic approach that comes to mind is like:
 * Going from left to right, my score from left side would be:
 * * 1 if I am lower or same height as the tree to the left
 * * the score of the tree before me + this score tells me how many places to jump next. So I jump and check the tree there:
 *  * if the next tree is still lower, add its score and keep jumping
 *  * if the next tree is higher or same height, that's it. Add the score and that's my score
 * 
 * Repeat for all directions. 
 */
fn part2() {
    let tree_map: Vec<Vec<u32>> = include_str!("../testinput")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).expect("NaN"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut score_map: Vec<Vec<HighestSeenYet>> = tree_map
        .iter()
        .map(|row| {
            vec![
                HighestSeenYet {
                    top: 0,
                    bot: 0,
                    right: 0,
                    left: 0
                };
                row.len()
            ]
        })
        .collect();

    let row_size = tree_map.len();
    
    // left side scores
    for i in 0..row_size {
        score_map[i][0].left = 0;
        for j in 1..row_size {
            if tree_map[i][j] <= tree_map[i][j-1] {
                score_map[i][j].left = 1;
                continue
            }

            let mut k = j-1;
            let mut score = 0;
            while k > 0 {
                // TODO: For now this is not working at all and there is an infinite loop here.

                for i in 0..row_size {
                    for j in 0..row_size {
                        print!("{}", score_map[i][j].left);
                    }
                    println!();
                }
                println!();


                if tree_map[i][k] >= tree_map[i][j] {
                    break;
                }

                score += score_map[i][k].left;
                k -= score_map[i][k].left as usize; 
            }
            score_map[i][j].left = score;
        }
    }


    println!("Input:");
    for i in 0..row_size {
        for j in 0..row_size {
            print!("{}", tree_map[i][j]);
        }
        println!();
    }
    println!();

    println!("From left:");
    for i in 0..row_size {
        for j in 0..row_size {
            print!("{}", score_map[i][j].left);
        }
        println!();
    }
    println!();
}

fn part2_brute() {
    let tree_map: Vec<Vec<u32>> = include_str!("../input")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).expect("NaN"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let forest_size = tree_map.len();
    let mut max_score = -1;
    for i in 1..forest_size-1 {
        for j in 1..forest_size-1 {
            let (mut left, mut right, mut top, mut bot) = (0, 0, 0, 0);
            for k in (0..j).rev() { //left 
                left += 1;
                if tree_map[i][k] >= tree_map[i][j] {
                    break;
                }
            }
            for k in (0..i).rev() { //top
                top += 1;
                if tree_map[k][j] >= tree_map[i][j] {
                    break;
                }
            }
            for k in j+1..forest_size { //right
                right += 1;
                if tree_map[i][k] >= tree_map[i][j] {
                    break;
                }
            }
            for k in i+1..forest_size { //bot
                bot += 1;
                if tree_map[k][j] >= tree_map[i][j] {
                    break;
                }
            }

            let score = left * right * top * bot;
            // println!("score[{}][{}] = ({}, {}, {}, {})", i, j, left, top, right, bot);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("{}", max_score);

}

fn main() {
    part2_brute();
}
