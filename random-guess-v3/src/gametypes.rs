//trait alias
pub trait GameTypeT : 
    num_traits::PrimInt + std::fmt::Display + std::ops::AddAssign {} //PrimInt needs to implement AddAssign trait for +=
impl<T> GameTypeT for T
    where T: num_traits::PrimInt + std::fmt::Display + std::ops::AddAssign {}

//type to actually use for GameType
pub type GameType = i32;