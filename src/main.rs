// extern crate ferris_says;
extern crate rand;

// use ferris_says::say;
use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // println!("Test 123");
    // say_hello();

    let secret: u32 = rand::thread_rng().gen_range(1..10);
    println!("Actually it's {}", secret);

    println!("Guess number!");
    loop {
        std::io::stdout().write(b"Yuor Input: ");
        std::io::stdout().flush();

        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error read line");

        let user_input = user_input.trim().parse::<u32>().expect("This is not number!");


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

// fn say_hello() {
//     let out = b"Hello fellow Rustaceans!";
//     let w = 24;
//
//     let mut writer: BufWriter<Stdout> = BufWriter::new(stdout());
//     say(out, w, &mut writer).unwrap();
// }