use indexmap::{indexmap, IndexMap};
use std::fs::read_to_string;
use std::path::Path;

fn create_word_to_num_mapping() -> IndexMap<String, String> {
    return indexmap! {
        "one".to_owned() => "1".to_owned(),
        "two".to_owned() => "2".to_owned(),
        "three".to_owned() => "3".to_owned(),
        "four".to_owned() => "4".to_owned(),
        "five".to_owned() => "5".to_owned(),
        "six".to_owned() => "6".to_owned(),
        "seven".to_owned() => "7".to_owned(),
        "eight".to_owned() => "8".to_owned(),
        "nine".to_owned() => "9".to_owned(),
    };
}

pub fn run() {
    // Read file
    let file_path = Path::new("./src/assets/day1/part_one.txt");
    let file_content = read_to_string(file_path).unwrap().to_string();

    // Create word to mapping
    let map = create_word_to_num_mapping();

    let mut numbers_to_sum: Vec<String> = vec![];

    // Iterate and change all occurences of word -> number
    for line in file_content.lines() {
        print!("Line: {}, ", line);
        let mut word = line.to_owned();

        loop {
            let mut index_map: IndexMap<usize, String> = IndexMap::new();

            for (_word, _) in &map {
                match word.find(_word) {
                    Some(position) => {
                        index_map.insert(position, _word.to_owned());
                    }
                    None => {}
                }
            }

            let mut keys: Vec<usize> = index_map.keys().cloned().collect();
            if keys.len() <= 0 {
                break;
            }
            keys.sort();

            // Sort by position and replace all occurences...
            // This won't match something that was partially replaced before
            for position in keys.iter() {
                let value = index_map.get(position).unwrap();
                word = word.replace(value, map.get(value).unwrap());
            }
        }

        print!(" -> {}", word);


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
        numbers_to_sum
            .push(format!("{}{}", first_number.unwrap(), second_number.unwrap()).to_string());
        println!(" -found-> {}{}", first_number.unwrap(), second_number.unwrap());
    }

    let mut final_sum: u64 = 0;

    // Sum them all up
    for number in numbers_to_sum {
        let number = number.parse::<u64>().unwrap();
        final_sum += number;
    }

    println!("The final sum will be: {}", final_sum);
}
