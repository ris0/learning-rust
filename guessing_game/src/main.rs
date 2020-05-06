use rand::Rng;
use std::cmp::Ordering;
use std::io;

// final draft
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

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

// second draft
// fn main() {


//     println!("The secret number is: {}", secret_number);
//     println!("Please input your guess.");
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
    
//     println!("You guessed: {}", guess);

// }

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// first draft
// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     // in rust, variables are immutable by default
//     // keyword mut is used to make it mutable
//     let mut guess = String::new();

//     // the :: syntax indicates that new is an associated function of the String type
//     // an associated function is impleneted on a type rather than on a particular instance.. aka static method
//     // std::io::stdin
//     // the & indicates that this argument is a reference, which gives you a way to let multiple parts 
//     // of your code access one piece of data without needing to copy that data into memory multiple times
//     // like variables, references are immutable by default
//     // te Result types are enumeration, fixed set of values
//     // those values are called the enum's variants
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//         // if you don't call expect, the program will compile but with warnings..
//         // right way to suppress the warning is to actually write error handling
    
//     // {} a place holder for value.
//     // you can print more than one value using curly brackets
//     // first set of curly brackets holds the first value listed after the format string
//     // println!("x = {} and y = {}", x, y);
//     println!("You guess: {}", guess);
// }
