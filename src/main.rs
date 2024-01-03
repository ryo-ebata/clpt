use std::env;
use std::fs::File;
use std::path::Path;

mod options;
mod systems;

#[derive(serde::Deserialize)]
struct Pet {
    r#type: String,
    name: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pet: Pet;

    match args.len() {
         // if user wrote command line with options
        n if n > 1 => {
            match args[1].as_str() {
                "--h" => options::help::help(),
                _ => println!("Invalid option. Try --h for help."),
            }
        },

        // if user wrote just command line
        1 => {
            if Path::new("src/public/clipet.json").exists() {
                let file = File::open("src/public/clipet.json").expect("Could not open file");
                match serde_json::from_reader::<_, Pet>(file) {
                    Ok(data) => {
                        println!("\nWELCOME BACK to CLI PET!\n");
                        pet = data
                    },
                    Err(_) => {
                        println!("The file does not contain valid Pet data.\n");
                        systems::first_choise::first_choise();

                        pet = fetch_pet();
                    }
                }
            } else {
                systems::first_choise::first_choise();
                pet = fetch_pet();
            }
            match pet.r#type.as_str() {
                "cat" => options::animals::cat::cat(&pet.name),
                "dog" => options::animals::dog::dog(&pet.name),
                "bird" => options::animals::bird::bird(&pet.name),
                _ => println!("Invalid option. Try --h for help."),
            }
        },
        _ => (),
    }
}

fn fetch_pet() -> Pet {
    let file = File::open("src/public/clipet.json").expect("Could not open file");
    serde_json::from_reader(file).expect("Failed to read JSON data")
}
