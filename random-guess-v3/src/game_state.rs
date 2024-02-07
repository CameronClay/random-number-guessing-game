
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