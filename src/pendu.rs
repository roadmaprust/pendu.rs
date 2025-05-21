mod game;

use rand::seq::SliceRandom;
use rpassword::read_password;
use std::fs::OpenOptions;
use std::io::{Write, stdin};
use game::Game;

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
        Game::print_actual_word(&game.get_output());
        println!("Erreurs restantes : {}", game.get_allowed_error());

        if game.has_won() {
            let score: usize = game.get_allowed_error() as usize * 10;
            println!(
                "🎉 Félicitations ! Tu as trouvé le mot : {}\n Score final {}",
                game.get_secret_word(), score
            );
            println!("Entrez votre nom : ");
            let mut nom = String::new();
            stdin().read_line(&mut nom).expect("Erreur de lecture");
            let nom = nom.trim();
            enregistrer_score(nom, score);
            break;
        } else if game.has_lost() {
            println!("💀 Dommage ! Le mot était : {}", game.get_secret_word());
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

        let char_teste = c.chars().next().unwrap();
        game.input(char_teste)
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
