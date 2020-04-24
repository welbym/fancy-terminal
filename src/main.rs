use colored::*;
use std::io;
use std::string::String;

// Runs a terminal program that can demonstrate
// how to change color of terminal output
fn main() {
    // Asks they user what they want the output
    // message to be
    println!("\nEnter your text:");

    // Mutable variable that holds input text
    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Did not enter a correct string");

    // how to look
    println!("Enter how you want it to look:");
    println!("Black, Red, Green, Yellow, Blue, Magenta, Cyan, or White");

    // Mutable variable that holds the input style
    let mut style = String::new();

    io::stdin()
        .read_line(&mut style)
        .expect("Did not enter a correct string");

    // get rid of white space then make lower case
    let style_trimmed = style.trim().to_ascii_lowercase();

    print!("\nYou typed:\n{}", text.color(style_trimmed));
}
