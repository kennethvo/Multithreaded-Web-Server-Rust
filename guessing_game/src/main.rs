use std::io; // importing libraries, this one being input output in standard library

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // var

    io::stdin() // user input. reads in val and error handles the result
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");
}
