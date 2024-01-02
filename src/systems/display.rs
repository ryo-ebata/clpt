use std::{thread, time};

pub fn display(seen1: &str, seen2: &str) {
    loop {
        println!("{}", seen1);
        thread::sleep(time::Duration::from_millis(500));
        println!("\x1B[2J\x1B[1;1H"); // Clear the console
        println!("{}", seen2);
        thread::sleep(time::Duration::from_millis(500));
        println!("\x1B[2J\x1B[1;1H"); // Clear the console
    }
}
