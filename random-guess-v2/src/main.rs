use std::io;
use std::str::FromStr; //for read_val concept (rust trait)
use std::io::Write; //for stdout().flush
use std::cmp::Ordering;
use rand::Rng;

// type ErrFmt = fn(&str) -> String;
// type ValPredicate<T> = fn(&T) -> bool;
// use generic parameter of type Fn rather than function pointer (fn) since anonymous functions with captures cannot be coerced to function pointer
fn read_val<T, ErrFmt, ValPredicate>(prompt: &str, error_fmt: ErrFmt, pred_opt: Option<ValPredicate>) -> T
    where T: FromStr,
          ErrFmt: Fn(&str) -> String,
          ValPredicate: Fn(&T) -> bool,
{
    loop {
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
                    Some(pred) if pred(&v) => return v,
                    None => return v,
                    Some(_) => print_err(),
                }
            },
            Err(_) => print_err(),
        };
    }
}

fn read_maxnum() -> i32 {
    read_val(
        "Enter a max number: ", 
        |_| format!("Invalid number, expected positive integer"),
        None::<fn(&i32) -> bool>
        //Some(|n| *n > 5)
    )
}

fn read_guess(minnum: i32, maxnum: i32) -> i32 {
    read_val(
        format!("Guess a number [{minnum}, {maxnum}]: ").as_str(), 
        |input| format!("Invalid guess: {input}, expected positive integer"),
        None::<fn(&i32) -> bool>
    )
}

fn main() {
    let minnum: i32 = 0;
    let maxnum: i32 = read_maxnum();
    let to_guess = rand::thread_rng().gen_range(minnum..=maxnum);

    let mut guess_cnt = 0u32;
    loop {
        guess_cnt += 1; // need increment here to include first guess
        let guess = read_guess(minnum, maxnum);
        if guess >= minnum && guess <= maxnum {
            match guess.cmp(&to_guess) {
                Ordering::Equal => break,
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too large"),
            }
        }
        else {
            println!("Number is within range: [{minnum}, {maxnum}]")
        }
    }
    println!("Correct! # of guesses: {guess_cnt}");
}