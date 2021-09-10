use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println! is a macro, which is defined by adding the ! at the end of println
    // if println were called without the !, then it would be calling a function
    println!("Guess the number!");

    // inclusive on the lower bound but exclusive on the upper bound
    // this could have also been written as 1..=100
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        // added mut to indicate the guess variable is mutable (can be changed)
        // by default all variables in rust are immutable (unchangable) unless otherwise specified
        // ::new indicates that new is an associated function of the String type
        //   an associated function is implemented on a type, in this case String
        // the new function created a new, empty string
        let mut guess = String::new();

        // we could have specified the following line as std::io::stdin if we didn't wanted to have use std::io as the first line
        io::stdin()
            // calls the read_line method on the standard input handle
            // the & indicates that the argument is a reference
            .read_line(&mut guess) 
            .expect("Failed to read line"); // handle errors if we are unable to read input from the standard input handle

        // reuse the guess variable name (shadow the previous value) to convert to u32 number
        // catch and print error if input is not convertable to u32
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // in this case it is better to handle invalid input from the user
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}