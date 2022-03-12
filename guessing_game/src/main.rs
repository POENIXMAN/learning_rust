use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Try to guess the number.");

    let secret_number = rand::thread_rng().gen_range(1,101);


    loop{

        println!("\nEnter a number: ");

        let mut answer = String::new();

        io::stdin().read_line(&mut answer).expect("Failed to read line");

        let answer: u32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match answer.cmp(&secret_number) {

            Ordering::Less => println!("\nToo small!"),
            Ordering::Greater => println!("\nToo big!"),
            Ordering::Equal => {

                println!("\nYou guessed it!\n");
                break;

                }

        }

    }

}
