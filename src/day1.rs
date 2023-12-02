use std::fs::read_to_string;
use std::path::Path;

pub fn run() {
    let mut lines: Vec<&str> = vec![];
    let file_path = Path::new("./src/assets/day1/part_one.txt");
    let file_content = read_to_string(file_path).unwrap().to_string();
    for line in file_content.lines() {
        lines.push(line);
    }

    let mut numbers_to_sum: Vec<String> = vec![];

    for word in lines.iter() {
        let word = word.to_string();
        let mut first_number: Option<char> = None;
        let mut second_number: Option<char> = None;

        for position in 0..word.len() {
            match word.chars().nth(position) {
                Some(number) if number.is_digit(10) => {
                    if first_number == None {
                        first_number = Some(number);
                        second_number = Some(number);
                    } else {
                        second_number = Some(number);
                    }
                }
                Some(_) | None => {}
            }
        }

        if first_number == None || second_number == None {
            println!("Did not find numbers for '{}'", word);
            continue;
        }
        println!(
            "Adding to sum: {}{}",
            first_number.unwrap(),
            second_number.unwrap()
        );
        numbers_to_sum
            .push(format!("{}{}", first_number.unwrap(), second_number.unwrap()).to_string());
    }

    let mut final_sum: u32 = 0;

    for number in numbers_to_sum {
        let number = number.parse::<u32>().unwrap();
        final_sum += number;
    }

    println!("The final sum will be: {}", final_sum);
}
