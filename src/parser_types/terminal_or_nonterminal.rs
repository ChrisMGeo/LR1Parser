use std::fmt;

use super::{nonterminal::NonTerminalTrait, terminal::TerminalTrait};

#[derive(Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum TerminalOrNonTerminal<
    Terminal: std::fmt::Debug
        + TerminalTrait
        + Copy
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + Ord
        + PartialOrd,
    NonTerminal: std::fmt::Debug
        + NonTerminalTrait
        + Copy
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + PartialOrd
        + Ord,
> {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}

impl<
        Terminal: std::fmt::Debug
            + TerminalTrait
            + Copy
            + Clone
            + PartialEq
            + std::hash::Hash
            + Eq
            + Ord
            + PartialOrd,
        NonTerminal: std::fmt::Debug
            + NonTerminalTrait
            + Copy
            + Clone
            + PartialEq
            + std::hash::Hash
            + Eq
            + PartialOrd
            + Ord,
    > fmt::Debug for TerminalOrNonTerminal<Terminal, NonTerminal>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonTerminal(arg0) => format!("{:?}", arg0).fmt(f),
            Self::Terminal(arg0) => format!("{:?}", arg0).fmt(f),
        }
    }
}
