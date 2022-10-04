mod images;
mod utils;
mod generator;

use crate::utils::check_files;
use crate::generator::{Collection, generate_all, set_output_dir};


fn main() {
    // make sure all the directories required are exists
    check_files().unwrap();
    set_output_dir().unwrap();

    let layers: Vec<&str> = vec!["Background", "Bottom lid", "Eye color", "Eyeball", "Goo", "Iris", "Shine", "Top lid"];

    generate_all(5, layers, Collection { name: "Deneme".to_string(), description: "Deneme".to_string(), base_uri: "https://deneme.com/".to_string()}).unwrap();
}
