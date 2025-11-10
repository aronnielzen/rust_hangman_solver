use std::io;
mod config;



fn word_vec_length_filter_via_user_input(word_vector: &Vec<String>) -> Vec<&str> {
    let mut new_word_vector = Vec::new();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let word_length: usize = user_input.trim().parse().expect("Please type a number!");

    for word in word_vector {
        if word.trim().len() == word_length {
            new_word_vector.push(word.as_str());
        }
    }
    new_word_vector
}

fn char_position_vec_builder_via_user_input(word_length: usize) -> Vec<(char, usize)> {
    let mut char_map: Vec<(char, usize)> = Vec::new();
    let mut user_input = String::new();
    let mut trimmed_user_input = String::new();

    while trimmed_user_input.len() != word_length || trimmed_user_input.is_empty() {
        println!(
            "Enter current known positions of the word, use _ for unknown letters (e.g. a__e_): "
        );

        user_input.clear();
        trimmed_user_input.clear();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        trimmed_user_input = user_input.trim().to_string();
    }

    let provided_positions: Vec<char> = trimmed_user_input.chars().collect();

    for i in 0..provided_positions.len() {
        if provided_positions[i] != '_' {
            char_map.push((provided_positions[i], i));
        }
    }
    char_map
}

fn position_evaluter(word: &str, char_positioning_map: &Vec<(char, usize)>) -> bool {
    for condition in char_positioning_map {
        if word.chars().nth(condition.1) != Some(condition.0) {
            return false;
        }
    }
    true
}

fn contains_char_checker(word: &str, guess_list: &Vec<char>) -> bool {
    for c in word.chars() {
        if guess_list.contains(&c) {
            return true;
        }
    }
    false
}

fn word_position_filter(
    char_position_vec: Vec<(char, usize)>,
    guess_list: Vec<char>,
    word_list: Vec<&str>,
) -> Vec<&str> {
    let mut filtered_word_list = Vec::new();

    for word in word_list {
        if !contains_char_checker(word, &guess_list) && position_evaluter(word, &char_position_vec)
        {
            filtered_word_list.push(word);
        }
    }
    filtered_word_list
}

fn guesser(guess_list: &Vec<char>, word_vector: &Vec<&str>, alphabet: &str) -> char {
    let mut highest_count = 0;
    let mut current_count = 0;
    let mut current_guess = alphabet.chars().nth(0).unwrap();

    for alphabetic_character in alphabet.chars().filter(|c| !guess_list.contains(c)) {
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

// let guess_list = tot_guess_list.iter().filter(|c| !char_position_vec.iter().any(|(ch, _)| ch == *c)).collect();

fn main() {
    println!("Welcome to the Hangman Solver!");
    println!("Enter the length of the word to guess:");

    let init_word_vector = word_vec_loader(config::WORD_LIST);
    let mut filtered_word_vector = word_vec_length_filter_via_user_input(&init_word_vector);
    let mut char_positioning_map: Vec<(char, usize)> = Vec::new();
    println!("Filtered words: {:?}", filtered_word_vector);

    while filtered_word_vector.len() > 1 {
        char_positioning_map =
            char_position_vec_builder_via_user_input(filtered_word_vector[0].len())
                .iter()
                .cloned()
                .collect();
        println!("Character positioning map: {:?}", char_positioning_map);
    }
}
