use std::io;
use std::str::FromStr; //for read_val concept (rust trait)
use std::io::Write; //for stdout().flush
use std::cmp::Ordering;
use rand::Rng;

type ErrFmt = fn(&str) -> String;
type ValPredicate<T> = fn(&T) -> bool;
fn read_val<T: FromStr>(prompt: &str, error_fmt: ErrFmt, pred_opt: Option<ValPredicate<T>>) -> T {
    let val: T = loop {
        print!("{prompt}"); //rust only flushes stdout if it encounters new line
        io::stdout().flush().unwrap();

        let mut val = String::new();
        io::stdin().read_line(&mut val).expect("Failed to read line");
        let val_result = val.trim().parse::<T>();

        let print_err = || {
            let error_prompt = error_fmt(&val);
            println!("{error_prompt}")
        };

        match val_result {
            Ok(v) => {
                match pred_opt {
                    Some(pred) if pred(&v) => break v,
                    None => break v,
                    Some(_) => print_err(),
                }
            },
            Err(_) => print_err(),
        };
    };
    val
}

fn read_maxnum() -> u32 {
    let maxnum: u32 = read_val(
        "Enter a max number: ", 
        |_| format!("Invalid number, expected positive integer"),
        None
        //Some(|n| *n > 5)
    );
    maxnum
}

fn read_guess(maxnum: u32) -> u32 {
    let guess: u32 = read_val(format!("Guess a number [0, {maxnum}]: ").as_str(), 
        |input| format!("Invalid guess: {input}, expected positive integer"),
        None
    );
    guess
}

fn main() {
    let maxnum: u32 = read_maxnum();
    let to_guess = rand::thread_rng().gen_range(0u32..=maxnum);

    let mut guess_cnt = 0u32;
    loop {
        match read_guess(maxnum).cmp(&to_guess) {
            Ordering::Equal => break,
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
        }
        guess_cnt += 1;
    }
    println!("Correct! # of guesses: {guess_cnt}");
}