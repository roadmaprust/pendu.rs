use rand::seq::SliceRandom;
use rpassword::read_password;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{Write, stdin};

fn main() {
    println!("📌 Bienvenue dans le jeu du pendu !");

    println!(
        "Choisis la difficulté : 1 - Facile (10 erreurs), 2 - Moyen (6 erreurs), 3 - Difficile (3 erreurs)"
    );
    let mut choix: String = String::new();
    stdin().read_line(&mut choix).expect("Erreur de lecture");

    let nb_erreur_autorise = match choix.trim() {
        "1" => 10,
        "2" => 6,
        "3" => 3,
        _ => 5,
    };

    println!("1️⃣ Mode Solo (mot aléatoire)");
    println!("2️⃣ Mode 2 joueurs (un joueur entre le mot secret)");

    let mut choix: String = String::new();
    print!("👉 Choisis une option (1 ou 2) : ");
    std::io::stdout().flush().unwrap(); // Force l'affichage immédiat

    stdin().read_line(&mut choix).expect("Erreur de lecture");
    let choix = choix.trim();

    let mot_a_deviner: String = if choix == "1" {
        choisir_mot_aleatoire()
    } else {
        print!("🔒 Entre le mot secret : ");
        std::io::stdout().flush().unwrap();
        let mot = read_password().expect("Erreur de lecture");
        mot.trim().to_string()
    };

    let mut game: Game = Game::init(mot_a_deviner, nb_erreur_autorise);

    loop {
        Game::print_actual_word(&game.output);
        println!("Erreurs restantes : {}", game.allowed_error);

        if game.has_won() {
            let score: usize = game.allowed_error as usize * 10;
            println!(
                "🎉 Félicitations ! Tu as trouvé le mot : {}\n Score final {}",
                game.secret_word, score
            );
            println!("Entrez votre nom : ");
            let mut nom = String::new();
            stdin().read_line(&mut nom).expect("Erreur de lecture");
            let nom = nom.trim();
            enregistrer_score(nom, score);
            break;
        } else if game.has_lost() {
            println!("💀 Dommage ! Le mot était : {}", game.secret_word);
            break;
        }

        let mut c: String = String::new();
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

fn choisir_mot_aleatoire() -> String {
    let mots = vec![
        "rust",
        "programmation",
        "sherli",
        "pendu",
        "jeu",
        "asynchrone",
    ];
    let mot_choisi = mots.choose(&mut rand::thread_rng()).unwrap();
    mot_choisi.to_string()
}

fn enregistrer_score(nom: &str, score: usize) {
    let mut fichier = OpenOptions::new()
        .append(true)
        .create(true)
        .open("scores.txt")
        .expect("Impossible d'ouvrir le fichier");

    writeln!(fichier, "{} : {}", nom, score).expect("Erreur d'écriture");
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
        assert!(game.char.contains(&'s'));
    }

    #[test]
    fn check_update_output_fn() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('s');
        assert_eq!(game.secret_word, "sherli".to_string());
        assert_eq!(game.allowed_error, 5);
        assert!(game.char.contains(&'s'));
        assert_eq!(game.output, "s_____".to_string());
    }

    #[test]
    fn check_input_fn_decrement_allowed_errors() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('z');
        assert_eq!(game.secret_word, "sherli".to_string());
        assert_eq!(game.allowed_error, 4);
        assert!(!game.char.contains(&'s'));
        assert_eq!(game.output, "______".to_string());
    }

    #[test]
    fn check_input_uppercase() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('S');
        assert_eq!(game.secret_word, "sherli".to_string());
        assert_eq!(game.allowed_error, 5);
        assert!(game.char.contains(&'s'));
        assert_eq!(game.output, "s_____".to_string());
    }

    #[test]
    fn check_has_won() {
        let mut game: Game = Game::init("sos".to_string(), 5);
        game.input('s');
        assert_eq!(game.secret_word, "sos".to_string());
        assert_eq!(game.allowed_error, 5);
        game.input('o');
        assert!(game.has_won());
    }

    #[test]
    fn check_has_lost() {
        let mut game: Game = Game::init("sos".to_string(), 1);
        assert_eq!(game.secret_word, "sos".to_string());
        game.input('t');
        assert_eq!(game.allowed_error, 0);
        game.input('r');
        assert!(game.has_lost());
    }
}
