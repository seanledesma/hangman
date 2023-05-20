use std::io;
use std::fs::File;
use std::io::Read;


fn main() {
    println!("WELCOME TO HANGMAN");
    body();
    //Get wordnum input from user
    let mut input = String::new();  //
    println!("Please enter a number for word length: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let word_num: i32 = input.trim().parse().expect("Invalid input");

    println!("You entered {}", word_num);






}

fn head() {
    println!("      _____");
    println!("     |     |");
    println!("     0     |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("-------------------");
}

fn torso() {
    println!("      _____");
    println!("     |     |");
    println!("     0     |");
    println!("     |     |");
    println!("     |     |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("-------------------");
}

fn arm1() {
    println!("      _____");
    println!("     |     |");
    println!("     0     |");
    println!("    /|     |");
    println!("   / |     |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("-------------------");
}

fn arm2() {
    println!("      _____");
    println!("     |     |");
    println!("     0     |");
    println!("    /|\\    |");
    println!("   / | \\   |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("-------------------");
}

fn leg1() {
    println!("      _____");
    println!("     |     |");
    println!("     0     |");
    println!("    /|\\    |");
    println!("   / | \\   |");
    println!("     ^     |");
    println!("    /      |");
    println!("   /       |");
    println!("           |");
    println!("-------------------");
}


fn body() {
    println!("      _____");
    println!("     |     |");
    println!("     0     |");
    println!("    /|\\    |");
    println!("   / | \\   |");
    println!("     ^     |");
    println!("    / \\    |");
    println!("   /   \\   |");
    println!("           |");
    println!("-------------------");
}