use std::fs;

pub fn help() {
    let paths = fs::read_dir("src/options/animals").unwrap();

    println!("\nCLI PET has the following options:\n");

    for path in paths {
        let file_name = path.unwrap().file_name();
        let file_name = file_name.to_string_lossy();
        if file_name != "mod.rs" {
            let file_name = file_name.replace(".rs", "");
            println!("--{}", file_name);
        }
    }
}
