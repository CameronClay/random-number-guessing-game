use super::guess::{Guess, GuessBounds, GuessState}; //relative import
use crate::gametypes::GameTypeT; //absolute import
use std::cmp;

pub struct GameState<T> 
    where T: GameTypeT
{
    guess_bounds: GuessBounds<T>,
    guess_count: T,
    to_guess: T
}

impl<T> GameState<T>
    where T: GameTypeT 
{
    pub fn new(guess_bounds: GuessBounds<T>, to_guess: T) -> Self {
        GameState { guess_bounds, guess_count:T::zero(), to_guess } //guess_bounds is short for guess_bounds:guess_bounds
    }

    pub fn get_guess_count(&self) -> T {
        self.guess_count
    }
    pub fn get_guess_bounds(&self) -> &GuessBounds<T> {
        return &self.guess_bounds;
    }
    pub fn get_to_guess(&self) -> T {
        return self.to_guess;
    }

    //Increments guess_count and returns a GuessState corresponding to the guess (incorrect or correct)
    pub fn process_guess(&mut self, guess: &Guess<T>) -> super::guess::GuessState { 
        self.guess_count = self.guess_count + T::one(); // self.guess_count + T::from(250).unwrap()
        let guess_val = guess.get_value();
        if guess_val >= self.guess_bounds.get_min() && guess_val <= self.guess_bounds.get_max() {
            match guess_val.cmp(&self.to_guess) {
                cmp::Ordering::Equal => {
                    return GuessState::Correct;
                },
                cmp::Ordering::Less => {
                    println!("Too small");
                    return GuessState::Incorrect;
                },
                cmp::Ordering::Greater => {
                    println!("Too large");
                    return GuessState::Incorrect;
                },
            }
        }
        else {
            println!("Number is within range: [{}, {}]", self.guess_bounds.get_min(), self.guess_bounds.get_max());
            return GuessState::Incorrect;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_guess_incorrect() {
        let mut game_state = GameState::new(GuessBounds::new(0, 100), 13);
        let state = game_state.process_guess(&Guess::new(12));
        assert!(state == GuessState::Incorrect);
    }

    #[test]
    fn test_process_guess_correct() {
        let mut game_state = GameState::new(GuessBounds::new(0, 100), 13);
        let state = game_state.process_guess(&Guess::new(13));
        assert!(state == GuessState::Correct);
    }

    #[test]
    fn test_get_guess_bounds() {
        let game_state = GameState::new(GuessBounds::new(0, 100), 13);
        let guess_bounds = game_state.get_guess_bounds();
        assert!(guess_bounds.get_min() == 0);
        assert!(guess_bounds.get_max() == 100);
    }

    #[test]
    fn test_get_guess_count_0() {
        let game_state = GameState::new(GuessBounds::new(0, 100), 13);
        assert!(game_state.get_guess_count() == 0);
    }

    #[test]
    fn test_get_guess_count_1() {
        let mut game_state = GameState::new(GuessBounds::new(0, 100), 13);
        game_state.process_guess(&Guess::new(12));
        assert!(game_state.get_guess_count() == 1);
    }

    #[test]
    fn test_get_to_guess() {
        let game_state = GameState::new(GuessBounds::new(0, 100), 13);
        let to_guess = game_state.get_to_guess();
        assert!(to_guess == 13);
    }
}