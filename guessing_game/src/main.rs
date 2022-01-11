use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("---Guessing Game---");
    let secret_num: usize = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Enter a number: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
                   .expect("Failed to read line.");
        println!("So, you guessed {}", guess);
        let guess: usize = guess.trim().parse().expect("Failed to convert");
        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too low !"),
            Ordering::Greater => println!("Too high !"),
            Ordering::Equal => {
                println!("That's a BINGO !");
                break;
            }
        }
    }
}
