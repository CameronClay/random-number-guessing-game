
//structs can access private members of another sturct if they are in the same module (e.g. source file)

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
