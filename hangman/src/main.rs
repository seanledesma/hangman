use std::io;
use std::io::Result;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use rand::Rng;


fn main() {
    
    let requested_word_length = welcome_screen();

    let mut lines = read_file();
    
    let random_number = random_number_generator();
    
    let hangman_word = find_right_length_word(requested_word_length, lines, random_number);
    
    game_loop(requested_word_length, hangman_word);

    /*
     *  TODO: 
     *  - clean up code, put into seperate game loop function
     *  - get rid of old game loop
     *  - figure out logic for ending point, if you lose or win
     */



}


fn game_loop(requested_word_length: i32, hangman_word: String) {
    //initialization
    let mut game_over: bool = false;
    let mut incorrect_attempts: i32 = 0;
    //let mut total_allowed_attempts: i32 = requested_word_length * 2;    //change this, shouldn't be based on attempts but on if you get hangman!
    //let mut attempts = total_allowed_attempts;
    let mut char_index: usize = 0;
    let mut letter_exists: bool = false;
    let mut hidden_word_vec: Vec<char> = Vec::new();

    //DEBUGGING, take out later
    println!("word: {}", hangman_word);
    println!();

    //place all '_' into word vector
    for h in 0..requested_word_length {
        let mut h_index = h as usize;
        hidden_word_vec.push('_');
        print!("{}", hidden_word_vec[h_index]);
    }
    println!();

    while !game_over {

        let user_char = get_user_char();
        char_index = 0;
        letter_exists = false;
        for c in hangman_word.chars() {
            
            if(user_char == c) {
                hidden_word_vec[char_index] = user_char;
                //println!("debuggin: {}", hidden_word_vec[char_index]);
                letter_exists = true;
            }
            char_index += 1;
        }

        if(!letter_exists){
            incorrect_attempts += 1;
        }
        //match is like switch statement, _ means base case
        match incorrect_attempts {
            0 => no_body(),
            1 => head(),
            2 => torso(),
            3 => arm1(),
            4 => arm2(),
            5 => leg1(),
            6 => body(),
            _ => println!("error in match statement"),
        }
        for k in 0..requested_word_length{
            let k_index = k as usize;
            print!("{}", hidden_word_vec[k_index]);
        }

        println!();
    }
}







fn welcome_screen() -> i32{
    println!("WELCOME TO HANGMAN");
    body();
    //Get wordnum input from user
    let mut input = String::new();  //
    println!("Enter a number for how long you would like your word to be: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let requested_word_length: i32 = input.trim().parse().expect("Invalid input");

    return requested_word_length;
}


fn read_file() -> Vec<String>{
    //open file, read file
    let file = File::open("words/words.txt").unwrap();
    let reader = BufReader::new(file);
    //create vector, read each line into vector
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines //return lines vec
}


fn random_number_generator() -> usize{
    //create a random number generator
    let mut rng = rand::thread_rng();
    //generate random number within specified range, i.e. length of text file
    let random_number = rng.gen_range(1..=2872); // TODO: change from hardcoded length to int

    random_number
}

fn find_right_length_word(requested_word_length: i32, lines: Vec<String>, random_number: usize) -> String{
    /*
    * The below code finds a word in the word vector that is the same 
    * length as the requested word length by the user.
    * Potential issues: 
    * If the random number takes you to the end of the list of words, there is a chance
    * you will not find the word with the right length
    */
    let mut hangman_word = String::new();
    for i in random_number..lines.len() {
        let random_word_length = lines[i].len() as i32; //this may be problematic, it returns num of bytes not char's
        if(random_word_length == requested_word_length) {
            //println!("Random word at random number: {}" , lines[i]);// TODO: fix so it can't go out of bounds
            hangman_word = lines[i].to_string();
            return hangman_word;
        }     
    }
    return hangman_word;
}


//swiped from GPT
fn get_user_char() -> char {
    let stdin = io::stdin();
    let mut input = String::new();

    println!("Please enter a letter:");
    stdin.lock().read_line(&mut input).expect("Failed to read line");

    let letter = input.trim().chars().next().expect("Empty input");
    letter
}


fn no_body() {
    println!("      _____");
    println!("     |     |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("           |");
    println!("-------------------");
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