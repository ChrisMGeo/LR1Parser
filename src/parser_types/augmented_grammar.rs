use super::{nonterminal::NonTerminalTrait, rule::Rule, terminal::TerminalTrait};

pub struct AugmentedGrammar<
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
    pub start_rule: Rule<Terminal, NonTerminal>,
    pub rules: Vec<Rule<Terminal, NonTerminal>>,
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
    > AugmentedGrammar<Terminal, NonTerminal>
{
    pub fn rules(&self) -> Vec<Rule<Terminal, NonTerminal>> {
        let mut rules = vec![self.start_rule.clone()];
        rules.extend(self.rules.iter().cloned());
        rules
    }
}
