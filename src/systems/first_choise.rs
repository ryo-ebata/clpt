use dialoguer::Select;
use serde_json::json;
use std::fs::File;
use std::io::{self};

/// This function prompts the user to choose a type of pet (cat, dog, bird) and enter a name for the pet.
/// The name should be up to 20 characters. If the name is longer, an error message is displayed and the function returns.
/// The chosen type and name are saved in a JSON file (`src/public/clipet.json`).
/// If the file is successfully created, a success message is displayed.
pub fn first_choise(){
    println!("Please choose one of the three animals:\n");

    let pet_types = &["cat", "dog", "bird"];
    let pet_type_selection = Select::new()
        .items(&pet_types[..])
        .default(0)
        .interact()
        .unwrap();

    let mut pet_name = String::new();

    println!("Please enter the name of the pet (up to 20 characters):\n");
    io::stdin().read_line(&mut pet_name).expect("Failed to read line\n");

    if pet_name.len() > 20 {
        println!("Pet name is too long. It should be up to 20 characters.\n");
        return;
    }

    let pet_json = json!({
        "type": pet_types[pet_type_selection],
        "name": pet_name.trim()
    });

    let file = File::create("src/public/clipet.json").expect("Could not create file\n");
    serde_json::to_writer(file, &pet_json).expect("Failed to write JSON data\n");

    println!("Good! You have a new CLI PET!!\n");
}
