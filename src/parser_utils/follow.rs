use std::collections::{BTreeMap, BTreeSet};

use crate::parser_types::{
    nonterminal::NonTerminalTrait, rule::Rule, terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

use super::first::compute_firsts;

#[allow(dead_code)]
pub fn compute_follows<
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
    computed_firsts: Option<&BTreeMap<NonTerminal, BTreeSet<Terminal>>>,
) -> BTreeMap<NonTerminal, BTreeSet<Terminal>> {
    let firsts = match computed_firsts {
        Some(f) => f.clone(),
        None => compute_firsts(rules),
    };
    let mut res = BTreeMap::new();
    let mut cloned = res.clone();
    for rule in rules {
        res.insert(rule.lhs, BTreeSet::new());
    }
    res.get_mut(&NonTerminal::start())
        .unwrap()
        .insert(Terminal::eof());
    while cloned != res {
        cloned = res.clone();
        for rule in rules {
            let a = rule.lhs;
            let follow_a = res.get(&a).cloned().unwrap();

            for index in 0..rule.rhs.len() {
                let _b = rule.rhs[index];
                if let TerminalOrNonTerminal::NonTerminal(b) = _b {
                    if index == rule.rhs.len() - 1 {
                        // rule3 A -> alpha B
                        res.get_mut(&b).unwrap().extend(&follow_a);
                    } else {
                        // rule2 A -> alpha B beta
                        let beta = rule.rhs[index + 1];
                        let first_beta = match beta {
                            TerminalOrNonTerminal::Terminal(t) => {
                                let mut set = BTreeSet::new();
                                set.insert(t);
                                set
                            }
                            TerminalOrNonTerminal::NonTerminal(nt) => firsts[&nt].clone(),
                        };
                        res.get_mut(&b).unwrap().extend(&first_beta);
                    }
                }
            }
        }
    }
    res
}
