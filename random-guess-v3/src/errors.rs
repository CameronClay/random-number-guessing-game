use std::fmt;
use std::marker::PhantomData;

pub trait InputError : fmt::Display {
    type Err<'a>: InputError;
    fn new<'a>(input: &'a str) -> Self::Err<'a>; 
}

impl InputError for MaxNumError<'_> {
    type Err<'a> = MaxNumError<'a>;
    fn new<'a>(input: &'a str) -> Self::Err<'a> {
        MaxNumError {input}
    }
}
impl InputError for GuessError<'_> {
    type Err<'a> = GuessError<'a>;
    fn new<'a>(input: &'a str) -> Self::Err<'a> {
        GuessError {input}
    }
}

pub struct MaxNumError<'a> {
    input: &'a str,
}
pub struct GuessError<'a> {
    input: &'a str,
}

impl<'a> fmt::Display for MaxNumError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid number, expected positive integer")
    }
}
impl<'a> fmt::Display for GuessError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid guess: {}, expected positive integer", self.input)
    }
}

// pub trait InputError : fmt::Display {}
// pub trait ErrorType {
//     type Err<T>;
//     fn new(input: &str) -> Self::Err<T>; //need 'a here because compiler complains associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`. I assume because of lifetime elision rule 3
// }

// pub struct MaxNumErrorT {}
// pub struct GuessErrorT {}

// impl<'a> ErrorType for MaxNumErrorT {
//     type Err<MaxNumError> = MaxNumError;
//     fn new(input: &'a str) -> Self::Err<MaxNumError> {
//         MaxNumError {input}
//     }
// }
// impl<'a> ErrorType for GuessErrorT {
//     type Err<GuessError> = GuessError;
//     fn new(input: &'a str) -> Self::Err<GuessError> {
//         GuessError {input}
//     }
// }

// pub struct MaxNumError<'a> {
//     input: &'a str,
// }
// pub struct GuessError<'a> {
//     input: &'a str,
// }

// impl<'a> fmt::Display for MaxNumError<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Invalid number, expected positive integer")
//     }
// }
// impl<'a> fmt::Display for GuessError<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Invalid guess: {}, expected positive integer", self.input)
//     }
// }