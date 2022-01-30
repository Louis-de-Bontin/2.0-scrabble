use std::io;


// TODO :
// Gérer le ex aeco
// Anoter le code

fn main() {
    let word1 = get_input(String::from("Player one : "));
    let word2 = get_input(String::from("Player two : "));

    let score_word1 = compute_score(&word1);
    let score_word2 = compute_score(&word2);

    println!("Player 1 : the score for \"{}\" is {}.", word1, score_word1);
    println!("Player 2 : the score for \"{}\" is {}.", word2, score_word2);
    let results = get_winner(score_word1, score_word2);
    println!("{}", results);
}

fn get_input(message: String) -> String {
    // This function displays an informative string to the user, and return his input
    println!("{}", message);
    let mut user_input = String::from("");
    io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input.");
    user_input = replace_char(&user_input, '\n');
    user_input
}

fn replace_char (init_str: &str, char_to_del: char) -> String {
    // Take a str and a char in input, return the String with the char deleted inside the String
    let mut out_str = String::from("");
    for character in init_str.chars() {
        if char_to_del != character {
            out_str.push(character);
        }
    }
    out_str
}

fn compute_score(word: &String) -> u32 {
    // Take a String ref in input and return the scrabble score
    let points = [1,   3,   3,   2,   1,   4,   2,   4,   1,   8,   5,   1,   3,   1,   1,   3,   10,  1,   1,   1,   1,   4,   4,   8,   4,   10];
    let lower = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    let word = word.to_lowercase(); // lowercase the word
    let mut score: u32 = 0;
    for letter in word.chars() {
        if lower.contains(&letter) {
            score += points[get_index(letter, lower)]; // Get the index a the char, and search 
                                                       //the corresponding index in points
        }
    }
    score
}

fn get_index(character: char, list: [char; 26]) -> usize {
    // Get the alphabetical position of a char.
    let index = list.iter().position(|&r| r == character).unwrap();
    index
}

fn get_winner(score1: u32, score2: u32) -> String {
    // Return the winnig quote
    if score1 > score2 {
        String::from("The winner is player 1.")
    } else if score2 > score1 {
        String::from("The winner is player 2.")
    } else {
        String::from("Exaeco !")
    }
}