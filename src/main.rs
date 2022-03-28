use std::env;
use std::fs;

fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    let content = fs::read_to_string(file_name).expect("Failed to read file");

    let dict = create_dict(&content);
    let content = create_content(&content, &dict);

    let mut file = &file_name[..file_name.rfind(".").expect("Failed to find period")];

    println!("{:?}", file);

    fs::write(file, content);
}

fn create_dict(content: &String) -> Vec<String> {
    let mut dict = Vec::new();
    let mut word = String::new();

    for char in content.chars() {
        if char.is_alphanumeric() {
            word.push(char);
        } else {
            if !(word == String::new()) {
                if !(dict.contains(&word)) {
                    dict.push(word.clone());
                }

                word = String::new();
            }
        }
    }

    dict
}

fn create_content(content: &String, dict: &Vec<String>) -> String {
    let mut compressed_content = String::new();
    let mut word = String::new();

    for char in content.chars() {
        if char.is_alphanumeric() {
            word.push(char);
        } else {
            if !(word == String::new()) {
                let pos = dict
                    .iter()
                    .position(|x| x == &word)
                    .expect("Failed to find word in dict");

                compressed_content.push_str(pos.to_string().as_str());

                word = String::new();
            }

            compressed_content.push(char);
        }
    }

    compressed_content
}
