fn main() {
    println!("Hello, world!");
}

const ROLLS: usize = 21;

struct Game {
    score: i16,
    rolls: [i16; ROLLS],
    current_roll: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            score: 0,
            rolls: [0; 21],
            current_roll: 0,
        }
    }

    fn update_score(&mut self, pins: i16) {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
    }

    fn get_score(mut self) -> i16 {
        for i in (0..self.rolls.len() - 1).step_by(2) {
            if self.is_strike(i) {
                self.score += self.strike_bonus(i);
            } else if self.is_spare(i) {
                self.score += self.spare_bonus(i);
            }
            self.score += self.frame_score(i);
        }
        if self.is_spare(19) || self.is_strike(19) {
            self.score += self.last_roll();
        }        
        self.score
    }

    fn last_roll(&self) -> i16 {
        self.rolls[self.rolls.len() - 1]
    }

    fn frame_score(&self, i: usize) -> i16 {
        self.rolls[i] + self.rolls[i + 1]
    }

    fn spare_bonus(&self, i: usize) -> i16 {
        self.rolls[i + 2]
    }

    fn strike_bonus(&self, i: usize) -> i16 {
        self.rolls[i+1] + self.rolls[i + 2]
    }

    fn is_strike(&self, i: usize) -> bool {
        self.rolls[i] == 10
    }

    fn is_spare(&self, i: usize) -> bool {
        self.rolls[i] + self.rolls[i + 1] == 10
    }
}

mod bowling_game_test;
