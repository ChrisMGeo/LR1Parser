// #[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
// pub enum NonTerminal {
//     Start,
//     Add,
//     Factor,
//     Term,
// }
// NonTerminal: NonTerminalTrait + Copy + Clone + PartialEq + std::hash::Hash + Eq + PartialOrd + Ord

#[allow(dead_code)]
pub trait NonTerminalTrait {
    fn is_start(&self) -> bool;
    fn start() -> Self;
}
