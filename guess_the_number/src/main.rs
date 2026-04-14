use rand::RngExt;
use std::{cmp::Ordering, io};

fn main() {
    println!("System Started!");

    let random_number: i32 = rand::rng().random_range(1..=100);

    loop {
        println!("Enter Your Number Please:-");
        let mut user_input: String = String::new(); // Heap mem

        io::stdin()
            .read_line(&mut user_input)
            .expect("Input Failed");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Enter a Vaild number");
                continue;
            }
        }; // Shadowing

        // match is exhaustive => must cover all cases
        match user_input.cmp(&random_number) {
            Ordering::Less => println!("Go Higher"),
            Ordering::Equal => {
                println!("You Get it !!!");
                break;
            }
            Ordering::Greater => println!("Go Lower"),
        };
    }
}
