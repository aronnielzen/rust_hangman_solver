use std::io;
mod words;




fn initial_word_sorter<'a>(word_length: usize, word_vector: &'a [&'static str]) -> Vec<&'a str> {
    let mut new_word_vector = Vec::new();

    for word in word_vector {
        if word.len() == word_length {
            new_word_vector.push(*word);
        } 
    }
    new_word_vector
}



fn char_guesser_from_word_list(alphabet: &str, word_vector: &Vec<&str>, guess_vector: &Vec<char>) -> char {
    let mut highest_count = 0;
    let mut current_count = 0;
    let mut current_guess = words::SWEDISH_ALPHABET.chars().nth(0).unwrap();

    for alphabetic_character in alphabet.chars().filter(|c| !guess_vector.contains(c)) {

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
            println!("Current highest count: {}", highest_count);
            println!("Current guess: {}", current_guess);
        }
    }
    current_guess
}





fn user_position_vec_converter(position_string: &str) -> Vec<(usize, char)> {
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

    println!("Welcome to the Hangman Solver!");

    let mut current_vec = initial_word_sorter(5, words::WORD_LIST);
    let mut guess_vector = Vec::new();
    let mut user_inputted_length = String::new();
    let mut user_input_map = String::new();

    let mut guess = char_guesser_from_word_list(words::SWEDISH_ALPHABET, &current_vec, &guess_vector);
    println!("{}", guess);

    guess_vector.push(guess);
    println!("{:?}", guess_vector);

    while current_vec.len() > 1 {

        println!("Enter the current map of the word, use _ for unknown letters (e.g. a__e_): ");
        
        io::stdin().read_line(&mut user_input_map);
        user_input_map = user_input_map.trim().to_string();
        println!("{}", user_input_map);

        guess = char_guesser_from_word_list(words::SWEDISH_ALPHABET, &current_vec, &guess_vector);
        println!("{}", guess);
        guess_vector.push(guess);
        println!("{:?}", guess_vector);


        let current_position_vec = user_position_vec_converter(&user_input_map);
        println!("{:?}", current_position_vec);
        current_vec = positional_word_sorter(&current_position_vec, &current_vec);
        println!("{:?}", current_vec);

    }
}