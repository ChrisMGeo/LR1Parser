use std::collections::{BTreeMap, BTreeSet};

use crate::parser_types::{
    nonterminal::NonTerminalTrait, rule::Rule, terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

pub fn first<
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
>(
    rules: &[Rule<Terminal, NonTerminal>],
    nt: &NonTerminal,
) -> BTreeSet<Terminal> {
    let mut res = BTreeSet::new();
    for rule in rules {
        if rule.lhs == *nt {
            match &rule.rhs[0] {
                TerminalOrNonTerminal::Terminal(t) => {
                    res.insert(*t);
                }
                TerminalOrNonTerminal::NonTerminal(new) => {
                    if new == nt {
                        continue;
                    }
                    res.extend(first(rules, new));
                }
            }
        }
    }
    res
}

pub fn compute_firsts<
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
>(
    rules: &[Rule<Terminal, NonTerminal>],
) -> BTreeMap<NonTerminal, BTreeSet<Terminal>> {
    let mut res = BTreeMap::new();
    for rule in rules {
        res.insert(rule.lhs, first(rules, &rule.lhs));
    }
    res
}
