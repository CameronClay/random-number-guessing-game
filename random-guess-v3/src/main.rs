mod gametypes;
mod guess;
mod game_state;
mod errors;

use std::io;
use std::str::FromStr; //for read_val concept (rust trait)
use std::io::Write; //for stdout().flush
use rand::Rng;

use errors::{InputError, MaxNumError, GuessError};
use guess::{Guess, GuessBounds, GuessState};
use game_state::GameState;
use gametypes::GameType;

// type ErrFmt = fn(&str) -> String;
// type ValPredicate<T> = fn(&T) -> bool;
// use generic parameter of type Fn rather than function pointer (fn) since anonymous functions with captures cannot be coerced to function pointer
fn read_val<ErrDisp, ValPredicate, T>(prompt: &str, pred_opt: Option<ValPredicate>) -> T
    where T: FromStr,
          ErrDisp: InputError,
          ValPredicate: Fn(&T) -> bool,
{
    loop {
        print!("{prompt}"); //rust only flushes stdout if it encounters new line
        io::stdout().flush().unwrap();

        let mut val = String::new();
        io::stdin().read_line(&mut val).expect("Failed to read line");
        let val_result = val.trim().parse::<T>();

        let print_err = || {
            println!("{}", ErrDisp::new(&val));
        };

        match val_result {
            Ok(v) => {
                match pred_opt {
                    Some(pred) if pred(&v) => return v,
                    None => return v,
                    Some(_) =>  println!("{}", ErrDisp::new(&val)),
                }
            },
            Err(_) => print_err(),
        };
    }
}

fn read_maxnum() -> i32 {
    read_val::<MaxNumError, _, _> (
        "Enter a max number: ", 
        None::<fn(&i32) -> bool>
    )
}

fn read_guess(minnum: i32, maxnum: i32) -> i32 {
    read_val::<GuessError, _, _> (
        format!("Guess a number [{minnum}, {maxnum}]: ").as_str(), 
        None::<fn(&i32) -> bool>
    )
}

fn game_loop() { //() is unit type aka void (implicit when -> is left out)
    let minnum: GameType = 0;
    let maxnum: GameType = read_maxnum();
    let to_guess = rand::thread_rng().gen_range(minnum..=maxnum);
    let guess_bounds = GuessBounds::new(minnum, maxnum);
    let mut game_state = GameState::new(guess_bounds, to_guess);

    loop {
        let guess = Guess::new(read_guess(minnum, maxnum));
        if game_state.process_guess(&guess) == GuessState::Correct {
            break;
        }
    }
    println!("Correct! # of guesses: {}", game_state.get_guess_count());
}

fn main() {
    game_loop();
}