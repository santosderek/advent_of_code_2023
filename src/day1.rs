use indexmap::{indexmap, IndexMap};
use std::fs::read_to_string;
use std::path::Path;

fn create_word_to_num_mapping() -> IndexMap<String, String> {
    let mut map = indexmap! {
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

    for num in 1..10 {
        map.insert(num.to_string(), num.to_string());
    }
    return map;
}

pub fn run() {
    // Read file
    let file_path = Path::new("./src/assets/day1/part_one.txt");
    let file_content = read_to_string(file_path).unwrap().to_string();

    // Create word to mapping
    let map = create_word_to_num_mapping();

    let mut final_sum: u64 = 0;

    // Iterate and change all occurences of word -> number
    for word in file_content.lines() {
        print!("Line: {}, ", word);

        let mut index_map: IndexMap<usize, String> = IndexMap::new();
        let mut keys: Vec<usize> = index_map.keys().cloned().collect();

        for (_word, _) in &map {
            let _ = word
                .match_indices(_word)
                .map(|(index, _)| {
                    index_map.insert(index.to_owned(), _word.to_owned());
                    index
                })
                .collect::<Vec<usize>>();
        }

        keys.sort();

        let (first_position, second_position): (&usize, &usize) = match (keys.first(), keys.last())
        {
            (Some(first), Some(last)) => (first, last),
            _ => continue,
        };

        let first_number: Option<&String> = map.get(index_map.get(first_position).unwrap());
        let second_number: Option<&String> = map.get(index_map.get(second_position).unwrap());

        let number = format!("{}{}", first_number.unwrap(), second_number.unwrap()).to_string();

        println!(" -found-> {}", number);

        let number = number.parse::<u64>().unwrap();

        final_sum += number;
    }

    println!("The final sum will be: {}", final_sum);
}
