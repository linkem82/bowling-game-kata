#[cfg(test)]
mod bowling_game {
    use crate::Game;

    #[test]
    fn test_gutter_game() {
        let mut game: Game = Game::new();
        let n = 20;
        let pins = 0;
        roll_many(&mut game, pins, n);
        assert_eq!(0, game.get_score());
    }

    #[test]
    fn test_all_ones() {
        let mut game: Game = Game::new();
        let n = 20;
        let pins = 1;
        roll_many(&mut game, pins, n);
        assert_eq!(20, game.get_score());
    }

    #[test]
    fn test_one_spare() {
        let mut game: Game = Game::new();
        roll_many(&mut game, 0, 18);
        game.update_score(3);
        roll_spare(&mut game);
        assert_eq!(13, game.get_score());
    }

    #[test]
    fn test_one_strike() {
        let mut game = Game::new();
        
        game.update_score(10);
        game.update_score(5);
        game.update_score(6);

        assert_eq!(32, game.get_score());
    }

    fn roll_many(game: &mut Game, pins: i16, times: i16) {
        for _i in 0..times {
            game.update_score(pins)
        }
    }

    fn roll_spare(game: &mut Game) {
        game.update_score(5);
        game.update_score(5);
    }
}
