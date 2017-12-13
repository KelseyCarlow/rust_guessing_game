extern crate rand;

use std::cmp::Ordering;
use rand::Rng;


fn main() {

    println!("Guess the number!");

    let mut min: u64 = 1;
    let mut max: u64 = 1000001;
    let mut counter: u32 = 0;

    let secret_number = rand::thread_rng().gen_range(min,max);

    loop {
        let guess = (min + max)/2;
        counter += 1;
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less      => {println!("Too Small"); min = guess },
            Ordering::Greater   => {println!("Too Big"); max = guess },
            Ordering::Equal     => {
                println!("You  win after {} attempts!", counter);
                break;
            }
        }
    }
}
