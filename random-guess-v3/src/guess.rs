//structs can access private members of another struct if they are in the same module (e.g. source file)

use crate::gametypes::GameTypeT; //absolute import
use std::fmt;

pub struct GuessBounds<T>
    where T: GameTypeT
{
    min: T, //private
    max: T,
}

impl<T> GuessBounds<T>
    where T: GameTypeT
{
    pub fn new(min: T, max: T) -> Self {
        assert!(min <= max, "Min must be less than max");
        GuessBounds { min, max }
    }

    pub fn get_min(&self) -> T {
        self.min
    }
    pub fn get_max(&self) -> T {
        self.max
    }
}

pub struct Guess<T>
    where T: GameTypeT
{
    value: T, //private
}

impl<T> Guess<T> 
    where T: GameTypeT
{
    pub fn new(value: T) -> Self {
        Guess { value }
    }

    pub fn get_value(&self) -> T {
        self.value
    }
}

#[derive(Eq, PartialEq)] //allows comparison of enum without pattern matching
pub enum GuessState {
    Correct,
    Incorrect
}

impl<T> fmt::Display for Guess<T>
    where T: GameTypeT
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_guess_get_value() {
        let guess = Guess::new(12);
        assert!(guess.get_value() == 12);
    }

    #[test]
    fn test_guess_bounds_get_min() {
        let bounds = GuessBounds::new(1, 10);
        assert!(bounds.get_min() == 1);
    }

    #[test]
    fn test_guess_bounds_get_max() {
        let bounds = GuessBounds::new(1, 10);
        assert!(bounds.get_max() == 10);
    }
}