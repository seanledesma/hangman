use std::io;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use rand::Rng;


fn main() -> std::io::Result<()>{
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
    //open file, read file
    let file = File::open("/Users/sean/Projects/programming_projects/rust_projects/hangman/words/words.txt")?;
    let reader = BufReader::new(file);
    //create vector, read each line into vector
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    //create a random number generator
    let mut rng = rand::thread_rng();
    //generate random number within specified range, i.e. length of text file
    let random_number = rng.gen_range(1..=2872); // TODO: change from hardcoded length to int
    //testing
    println!("Random number: {} " , random_number);
    

    let random_word_length: i32 = lines[random_number].len(); //this may be problematic, it returns num of bytes not char's

    println!("random word length: {}" , random_word_length);

    if(word_num == random_word_length) {
        println!("Random word at random number: {}" , lines[random_number]);// TODO: fix so it can't go out of bounds
    }








    Ok(())
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