use std::fmt;

use super::{
    nonterminal::NonTerminalTrait, terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

#[derive(Clone, PartialEq)]
pub struct Rule<
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
    pub lhs: NonTerminal,
    pub rhs: Vec<TerminalOrNonTerminal<Terminal, NonTerminal>>,
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
    > fmt::Debug for Rule<Terminal, NonTerminal>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rhs = self.rhs.clone();
        let rhs_mapped: Vec<String> = rhs
            .into_iter()
            .map(|s| match s {
                TerminalOrNonTerminal::Terminal(t) => format!("{:?}", t),
                TerminalOrNonTerminal::NonTerminal(nt) => format!("{:?}", nt),
            })
            .collect();
        let rhs_str = rhs_mapped.join(" ");
        write!(f, "{:?} -> {}", self.lhs, rhs_str)
    }
}
