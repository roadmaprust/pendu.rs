use rpassword::read_password;
use std::collections::HashSet;
use std::io::stdin;

fn main() {
    println!("Entre le mot secret : ");
    let mot_a_deviner = read_password().expect("erreur de lecture du mot secret");

    // Nettoyer le mot secret
    let mot_a_deviner = mot_a_deviner.trim().to_string();

    let mut game = Game::init(mot_a_deviner, 5);

    loop {
        Game::print_actual_word(&game.output);
        println!("Erreurs restantes : {}", game.allowed_error);

        if game.has_won() {
            println!(
                "🎉 Félicitations ! Tu as trouvé le mot : {}",
                game.secret_word
            );
            break;
        } else if game.has_lost() {
            println!("💀 Dommage ! Le mot était : {}", game.secret_word);
            break;
        }

        let mut c = String::new();
        println!("Entre un caractère : ");
        stdin().read_line(&mut c).expect("Erreur de lecture");

        let c = c.trim(); // Nettoyer l'entrée (shadowing)
        if c.len() != 1 || !c.chars().next().unwrap().is_alphabetic() {
            println!("❌ Entrée invalide ! Entre une seule lettre.");
            continue;
        }

        let char_testé = c.chars().next().unwrap();
        game.input(char_testé)
    }
}

struct Game {
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

    fn print_actual_word(mot: &str) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_game() {
        let game = Game::init("sherli".to_string(), 3);
        assert_eq!(game.secret_word, "sherli".to_string());
        assert_eq!(game.allowed_error, 3);
        assert_eq!(game.output, "______".to_string());
    }

    #[test]
    fn check_input_fn_modify_hashset() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('s');
        assert_eq!(game.secret_word, "sherli".to_string());
        assert_eq!(game.allowed_error, 5);
        assert_eq!(game.char.contains(&'s'), true);
    }

    #[test]
    fn check_update_output_fn() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('s');
        assert_eq!(game.secret_word, "sherli".to_string());
        assert_eq!(game.allowed_error, 5);
        assert_eq!(game.char.contains(&'s'), true);
        assert_eq!(game.output, "s_____".to_string());
    }

    #[test]
    fn check_input_fn_decrement_allowed_errors() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('z');
        assert_eq!(game.secret_word, "sherli".to_string());
        assert_eq!(game.allowed_error, 4);
        assert_eq!(game.char.contains(&'s'), false);
        assert_eq!(game.output, "______".to_string());
    }

    #[test]
    fn check_input_uppercase() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('S');
        assert_eq!(game.secret_word, "sherli".to_string());
        assert_eq!(game.allowed_error, 5);
        assert_eq!(game.char.contains(&'s'), true);
        assert_eq!(game.output, "s_____".to_string());
    }
}
