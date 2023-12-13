use std::collections::HashMap;
use std::fs;

fn main() {
    let file_content = fs::read_to_string("src/aoc.txt");
    let mut total = 0;

    for line in file_content.unwrap().lines() {
        total += get_points(line);
    }

    println!("{}", total);

}

fn get_points(line: &str) -> i32 {
    let mut add = 0;
    let card_data: Vec<_> = line.split(':').collect();
    let numbers = card_data[1];
    let all_numbers: Vec<_> = numbers.split('|').collect();
    let winning_nums = all_numbers[0];
    let player_nums = all_numbers[1];
    let winners: Vec<_> = winning_nums.split(' ').collect();
    let played: Vec<&str> = player_nums.split(' ').collect();
    let mut mp: HashMap<String, bool> = HashMap::new();
    for winner in winners {
        let w = winner.replace(" ", "");
        if w != "" {
            mp.insert(w, true);
        }
    }

    for play in played {
        if mp.contains_key(play) {
            if add == 0 {
                add += 1;
            } else {
                add *= 2;
            }
        }
    }

    add
}
