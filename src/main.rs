use std::env;
use std::fs;

fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    let data = fs::read_to_string(file_name).expect("Failed to read file");

    let mut created_file_name =
        file_name[..file_name.rfind(".").expect("Failed to find period")].to_string();
    created_file_name.push_str(".bgn");

    let dict = create_dict(&data);
    let content = create_content(data, &dict);

    let mut full_content = String::from("[");
    full_content.push_str(dict.join(",").as_str());
    full_content.push_str("]\n\n");
    full_content.push_str(content.as_str());

    fs::write(created_file_name, full_content).expect("Failed to write to file");
}

fn create_dict(data: &String) -> Vec<String> {
    let mut dict = Vec::new();
    let mut word = String::new();

    for char in data.chars() {
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

fn create_content(data: String, dict: &Vec<String>) -> String {
    let mut compressed_content = String::new();
    let mut word = String::new();

    for char in data.chars() {
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
