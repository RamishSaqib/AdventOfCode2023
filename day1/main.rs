use std::fs::File;
use std::io::Read;

fn main() {
    let mut data_file = File::open("src/aoc.txt").unwrap();

    let mut file_content = String::new();

    data_file.read_to_string(&mut file_content).unwrap();

    let parts = file_content.split('\n');

    let mut sum  = 0;

    for part in parts {
        let mut first_num: char = ' ';
        let mut last_num: char = ' ';

        for c in part.chars() {
            if c.is_numeric() {
                if first_num == ' ' {
                    first_num = c;
                } else {
                    last_num = c;
                }
            }
        }

        if last_num == ' ' {
            last_num = first_num;
        }

        let mut s = String::from(first_num);
        s.push(last_num);

        sum += match s.parse::<i32>() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Error parsing string: {}", err);
                continue;
            }
        }
    }

    println!("The total is {}", sum);
}
