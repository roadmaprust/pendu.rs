use game::Game;

#[test]
fn init_game() {
    let game = Game::init("sherli".to_string(), 3);
    assert_eq!(game.get_secret_word(), "sherli".to_string());
    assert_eq!(game.get_allowed_error(), 3);
    assert_eq!(game.get_output(), "______".to_string());
}

#[test]
fn check_input_fn_modify_hashset() {
    let mut game: Game = Game::init("sherli".to_string(), 5);
    game.input('s');
    assert_eq!(game.get_secret_word(), "sherli".to_string());
    assert_eq!(game.get_allowed_error(), 5);
    assert!(game.get_char().contains(&'s'));
}

    #[test]
    fn check_update_output_fn() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('s');
        assert_eq!(game.get_secret_word(), "sherli".to_string());
        assert_eq!(game.get_allowed_error(), 5);
        assert!(game.get_char().contains(&'s'));
        assert_eq!(game.get_output(), "s_____".to_string());
    }

    #[test]
    fn check_input_fn_decrement_allowed_errors() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('z');
        assert_eq!(game.get_secret_word(), "sherli".to_string());
        assert_eq!(game.get_allowed_error(), 4);
        assert!(!game.get_char().contains(&'s'));
        assert_eq!(game.get_output(), "______".to_string());
    }

    #[test]
    fn check_input_uppercase() {
        let mut game: Game = Game::init("sherli".to_string(), 5);
        game.input('S');
        assert_eq!(game.get_secret_word(), "sherli".to_string());
        assert_eq!(game.get_allowed_error(), 5);
        assert!(game.get_char().contains(&'s'));
        assert_eq!(game.get_output(), "s_____".to_string());
    }

    #[test]
    fn check_has_won() {
        let mut game: Game = Game::init("sos".to_string(), 5);
        game.input('s');
        assert_eq!(game.get_secret_word(), "sos".to_string());
        assert_eq!(game.get_allowed_error(), 5);
        game.input('o');
        assert!(game.has_won());
    }

    #[test]
    fn check_has_lost() {
        let mut game: Game = Game::init("sos".to_string(), 1);
        assert_eq!(game.get_secret_word(), "sos".to_string());
        game.input('t');
        assert_eq!(game.get_allowed_error(), 0);
        game.input('r');
        assert!(game.has_lost());
    }
