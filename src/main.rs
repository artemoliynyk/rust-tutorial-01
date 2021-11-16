// extern crate ferris_says;
extern crate rand;

use ferris_says::say;
use std::io;
use std::io::{BufWriter, Stdout, stdout, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    write_into("Welcome to the guess game!");

    let secret: u32 = rand::thread_rng().gen_range(1..10);
    println!("Actually it's {}", secret);

    println!("Guess number!");
    loop {
        std::io::stdout().write(b"\nYour Input: ");
        std::io::stdout().flush();

        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error read line");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(i) => i,
            Err(e) => {
                say_its_not_ok(e.to_string());
                continue;
            }
        };

        match user_input.cmp(&secret) {
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too much")
        }
    }
}

fn write_into(text: &str) {
    let mut writer: BufWriter<Stdout> = BufWriter::new(stdout());
    say(text.as_bytes(), text.len(), &mut writer).unwrap();
}

fn say_its_not_ok(error_msg: String) {
    let phrases = &mut ["Sorry, what is it?", "Huh?", "No, not like this",
        "Come-on, stop it!", "But for real, is it a number?"];
    let p_index: usize = rand::thread_rng().gen_range(std::ops::Range { start: 0, end: phrases.len() });

    println!("{} ({})", phrases[p_index], error_msg);
}