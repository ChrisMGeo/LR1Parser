// #[derive(Copy, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
// pub enum Terminal {
//     Eof,
//     LeftParen,
//     Name,
//     Int,
//     RightParen,
//     Plus,
//     Asterisk,
// }
// impl fmt::Debug for Terminal {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Name => write!(f, "name"),
//             Self::Int => write!(f, "int"),
//             Self::LeftParen => write!(f, "("),
//             Self::RightParen => write!(f, ")"),
//             Self::Plus => write!(f, "+"),
//             Self::Asterisk => write!(f, "*"),
//             Self::Eof => write!(f, "$"),
//         }
//     }
// }
// Terminal: TerminalTrait + Copy + Clone + PartialEq + std::hash::Hash + Eq + Ord + PartialOrd

pub trait TerminalTrait {
    fn is_eof(&self) -> bool;
    fn eof() -> Self;
}
