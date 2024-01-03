use std::{thread, time};
use std::io::{self, Write};

fn clear_lines(n: usize) {
    print!("\x1B[{}A", n); // Move the cursor up n lines
    io::stdout().flush().unwrap(); // Ensure the output is flushed
    for _ in 0..n {
        print!("\x1B[2K"); // Clear the current line
        print!("\x1B[1B"); // Move the cursor down 1 line
    }
    print!("\x1B[{}A", n); // Move the cursor up n lines again
    io::stdout().flush().unwrap(); // Ensure the output is flushed
}

pub fn display_pet(seen1: &str, seen2: &str, name: &str, lines: usize) {
    println!("{}", "name: ".to_string() + name);
    loop {
        println!("{}", seen1);
        thread::sleep(time::Duration::from_millis(500));
        clear_lines(lines);
        println!("{}", seen2);
        thread::sleep(time::Duration::from_millis(500));
        clear_lines(lines);
    }
}
