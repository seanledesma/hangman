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
    println!("How many digits would you like your word to be? ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let requested_word_length: i32 = input.trim().parse().expect("Invalid input");

    println!("You entered {}", requested_word_length);
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
    
    
/*
 * The below code finds a word in the word vector that is the same 
 * length as the requested word length by the user.
 * Potential issues: 
 * If the random number takes you to the end of the list of words, there is a chance
 * you will not find the word with the right length
 */
    for i in random_number..lines.len() {
        let random_word_length: i32 = lines[i].len().try_into().unwrap(); //this may be problematic, it returns num of bytes not char's
        if(random_word_length == requested_word_length) {
            println!("Random word at random number: {}" , lines[i]);// TODO: fix so it can't go out of bounds
            break;
        }     
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