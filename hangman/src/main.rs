use std::io;
mod config;


fn word_vec_loader(wordlist: &&[&str]) -> Vec<String> {
    let mut word_vector = Vec::new();

    for word in wordlist.iter() {
        word_vector.push((*word).into());
    }
    word_vector
}


fn word_vec_length_filter_via_user_input(word_vector: &Vec<String>) -> Vec<&str> {
    let mut new_word_vector = Vec::new();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let word_length: usize = user_input.trim().parse().expect("Please type a number!");

    for word in word_vector {
        if word.trim().len() == word_length {
            new_word_vector.push(word.as_str());
        } 
    }
    new_word_vector
}


fn word_heatmap_builder_via_user_input(word_length: usize) -> Vec<(char, usize)> {
    let mut char_map: Vec<(char, usize)> = Vec::new();
    let mut user_input = String::new();
    let mut trimmed_user_input = String::new();

    while trimmed_user_input.len() != word_length || trimmed_user_input.is_empty() {
        println!("Enter current known positions of the word, use _ for unknown letters (e.g. a__e_): ");

        user_input.clear();
        trimmed_user_input.clear();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");
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


fn main() {
    println!("Welcome to the Hangman Solver!");
    println!("Enter the length of the word to guess:");

    let word_list = config::WORD_LIST;
    let word_vector = word_vec_loader(&word_list);
    let mut filtered_words = word_vec_length_filter_via_user_input(&word_vector);
    let mut char_positioning_map: Vec<(char, usize)> = Vec::new();
    println!("Filtered words: {:?}", filtered_words);

    while filtered_words.len() > 1 {
        char_positioning_map.push(word_heatmap_builder_via_user_input(filtered_words[0].len()).pop().unwrap());
        println!("Character positioning map: {:?}", char_positioning_map);
    }
}