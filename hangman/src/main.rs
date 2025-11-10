mod config;

fn word_vec_loader(wordlist: &[&str]) -> Vec<String> {
    let mut word_vector = Vec::new();

    for word in wordlist.iter() {
        word_vector.push((*word).into());
    }
    return word_vector;
}

fn word_vec_length_filter<'a>(word_vec: &'a Vec<String>, length: &'a usize) -> Vec<&'a str> {
    let mut filtered_vec = Vec::new();

    for word in word_vec.iter() {
        if word.len() == *length {
            filtered_vec.push(word.as_str())
        }
    }
    return filtered_vec;
}

fn char_position_map_builder(user_input: &str) -> Vec<(char, usize)> {
    let mut pos_map = Vec::new();
    for i in 0..user_input.len() {
        if user_input.chars().nth(i).unwrap() != '_' {
            pos_map.push((user_input.chars().nth(i).unwrap(), i))
        }
    }
    return pos_map;
}

fn user_position_input_checker(user_input: &str, alphabet: &str) -> bool {
    for c in user_input.chars() {
        if alphabet.contains(c) {
        } else {
            return false;
        }
    }
    return true;
}

fn word_tuple_checker(
    word: &str,
    position_vec: &Vec<(char, usize)>,
    already_guessed_list: &Vec<char>,
) -> bool {
    for c in already_guessed_list {
        if word.contains(*c) {
        } else {
            return false;
        }
    }

    for t in position_vec {
        if word.chars().nth(t.1).unwrap() == t.0 {
        } else {
            return false;
        }
    }
    return true;
}

fn word_vec_char_filter<'b>(
    word_vec: &Vec<&'b str>,
    char_position_map: &'b Vec<(char, usize)>,
    already_guessed_list: &'b Vec<char>,
) -> Vec<&'b str> {
    let mut filtered_vec = Vec::new();
    for word in word_vec {
        if word_tuple_checker(word, char_position_map, already_guessed_list) {
            filtered_vec.push(*word)
        }
    }
    return filtered_vec;
}

fn most_common_word_finder(word_vec: &Vec<&str>, alphabet: &str, already_guessed_list: &Vec<char>) -> char {
    let mut counter = 0;
    let mut current_char = alphabet.chars().nth(0).unwrap();
    for c in alphabet {
        if //c finns i already_guessed
        // denna logik finns redan i tertiary
    }
}

fn main() {
    let ag = vec!['g','d'];
    let init_vec = word_vec_loader(config::WORD_LIST);
    let mut filtered_vec = word_vec_length_filter(&init_vec, &10);
    let position = char_position_map_builder("a___c");
    filtered_vec = word_vec_char_filter(&filtered_vec, &position, &ag);
    println!("{:?}", filtered_vec)
}
