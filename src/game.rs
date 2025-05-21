#![allow(dead_code)]
use std::collections::HashSet;

pub struct Game {
    secret_word: String,
    output: String,
    char: HashSet<char>,
    allowed_error: u8,
}

impl Game {
    pub fn init(mot_a_deviner: String, nb_erreur_autorise: u8) -> Self {
        Self {
            secret_word: mot_a_deviner.clone(),
            char: HashSet::new(),
            output: "_".repeat(mot_a_deviner.len()),
            allowed_error: nb_erreur_autorise,
        }
    }

    pub fn input(&mut self, input: char) {
        let input = input.to_ascii_lowercase();

        if self.char.contains(&input) {
            println!("🔁 Lettre déjà testée !");
            return;
        }

        self.char.insert(input);

        if self.secret_word.contains(input) {
            self.update_output(input);
        } else {
            self.allowed_error = self.allowed_error.saturating_sub(1);
            println!("❌ Mauvaise lettre !");
        }
    }

    pub fn update_output(&mut self, c: char) {
        let mut output_char: Vec<char> = self.output.chars().collect();
        for (i, letter) in self.secret_word.chars().enumerate() {
            if letter == c {
                output_char[i] = c;
            }
        }
        self.output = output_char.into_iter().collect();
    }

    pub fn print_actual_word(mot: &str) {
        println!(
            "\nMot actuel : {}",
            mot.chars().map(|c| format!("{} ", c)).collect::<String>()
        )
    }

    pub fn has_won(&self) -> bool {
        self.output == self.secret_word
    }

    pub fn has_lost(&self) -> bool {
        self.allowed_error == 0
    }

    pub fn get_secret_word(&self) -> &str {
        &self.secret_word
    }

    pub fn set_secret_word(&mut self, secret_word: String) {
        self.secret_word = secret_word;
    }

    pub fn get_output(&self) -> &str {
        &self.output
    }

    pub fn set_output(&mut self, output: String) {
        self.output = output;
    }

    pub fn get_char(&self) -> &HashSet<char> {
        &self.char
    }

    pub fn set_char(&mut self, chars: HashSet<char>) {
        self.char = chars;
    }

    pub fn get_allowed_error(&self) -> u8 {
        self.allowed_error
    }

    pub fn set_allowed_error(&mut self, allowed_error: u8) {
        self.allowed_error = allowed_error;
    }
}