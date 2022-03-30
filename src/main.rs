use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() {
    let file_name = &env::args().collect::<Vec<String>>()[1];

    match Path::new(file_name).extension().and_then(OsStr::to_str) {
        // Some("bgn") => decompress(file_name),
        _ => compress(file_name),
    }
}

fn compress(file_name: &String) {
    let data = fs::read_to_string(file_name).expect("Failed to read file");

    let dict = create_dict(&data);
    let content = create_content(data, &dict);

    let final_name = [&file_name[..file_name.rfind(".").unwrap()], "bgn"].join(".");
    let full_content = [dict.join(","), content].join("\n");

    fs::write(final_name, full_content).expect("Failed to write to file");
}

fn create_dict(data: &String) -> Vec<String> {
    const NEW_STRING: String = String::new();

    let mut dict = Vec::new();
    let mut word = String::new();

    for char in data.chars() {
        if char.is_alphabetic() {
            word.push(char);
        } else {
            if !(word == NEW_STRING) {
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
    let mut content = String::new();
    let mut word = String::new();

    for char in data.chars() {
        if char.is_alphabetic() {
            word.push(char);
        } else {
            if !(word == String::new()) {
                let pos = dict
                    .iter()
                    .position(|dict_word| dict_word == &word)
                    .unwrap();

                content.push_str(pos.to_string().as_str());

                word = String::new();
            }

            content.push(char);
        }
    }

    content
}
