use std::io;
mod words;


// Finds most common character in a list of words
fn char_guesser(alphabet: &str, word_vector: &Vec<&str>) -> char {
    let mut highest_count = 0;
    let mut current_count = 0;
    let mut current_guess = 'a';

    for alphabetic_character in alphabet.chars() {

        current_count = 0;

        for word in word_vector {
            for character in word.chars() {
                if character == alphabetic_character {
                    current_count += 1;
                }
            }
        }

        if current_count > highest_count {
            highest_count = current_count;
            current_guess = alphabetic_character;
        }
    }
    current_guess
}



// Returns a list of references to words with conformed length, used to initialize the word list
fn initial_word_sorter<'a>(word_length: usize, word_vector: &'a [&'static str]) -> Vec<&'a str> {
    let mut new_word_vector = Vec::new();

    for word in word_vector {
        if word.len() == word_length {
            new_word_vector.push(*word);
        } 
    }
    new_word_vector
}

// Converts user inputted string to vector of (position, character) tuples
fn position_vec_converter(position_string: &str) -> Vec<(usize, char)> {
    let mut position_vector = Vec::new();

    for i in 0..position_string.len() {
        let char_at_pos = position_string.chars().nth(i);
        if char_at_pos != Some('_') {
            position_vector.push((i, char_at_pos.unwrap()));
        }
    }
    position_vector
}


// Evaluates if a word has correct character placement
fn position_evaluter(word: &str, char_positioning_map: &Vec<(usize, char)>) -> bool {
    for condition in char_positioning_map {
        if word.chars().nth(condition.0) != Some(condition.1) {
            return false;
        }
    }
    true  
}


// Returns a list of references to words with correct character placement
fn positional_word_sorter<'b>(char_positioning_map: &Vec<(usize, char)>, word_vector: &Vec<&'b str>) -> Vec<&'b str> {
    let mut new_word_vector = Vec::new();

    for word in word_vector {
        if position_evaluter(word, char_positioning_map) {
            new_word_vector.push(*word);
        }
    }
    new_word_vector
}



fn main() {
    println!("Welcome to Hangman Solver!\nPlease input word length:");
    let mut user_input_word_length = String::new();
    io::stdin().read_line(&mut user_input_word_length)
        .expect("Failed to read line");
    let word_length = user_input_word_length.trim().parse::<usize>()
        .expect("Please enter a valid number");



    let current_vec = initial_word_sorter(word_length, &words::WORD_LIST);

    while true {

        println!("Enter known positions (use _ for unknowns, e.g. s_a for s in first position and a in third):");
        let mut position_input = String::new();


        io::stdin().read_line(&mut position_input)
            .expect("Failed to read line");


        if position_input != "\n" {
            let position_map_vec = position_vec_converter(&position_input);
            let current_vec = positional_word_sorter(&position_map_vec, &current_vec);
            let guess = char_guesser(words::SWEDISH_ALPHABET, &current_vec);
            println!("Possible words: {:?}", current_vec);
            println!("Best guess: {}", guess);
        }
        else {
            let guess = char_guesser(words::SWEDISH_ALPHABET, &current_vec);
            println!("Possible words: {:?}", current_vec);
            println!("Best guess: {}", guess);
        }
    }
    

    
}
