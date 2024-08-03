use std::collections::{BTreeMap, BTreeSet};

use crate::parser_types::{
    lr1item::LR1Item, nonterminal::NonTerminalTrait, rule::Rule, terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

use super::first::compute_firsts;

pub fn lr1_closure<
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
    kernel: &[LR1Item<Terminal, NonTerminal>],
    precomputed_firsts: Option<&BTreeMap<NonTerminal, BTreeSet<Terminal>>>,
) -> BTreeSet<LR1Item<Terminal, NonTerminal>> {
    let first = match precomputed_firsts {
        Some(f) => f.clone(),
        None => compute_firsts(rules),
    };
    let mut res = BTreeSet::new();
    for item in kernel {
        res.insert(item.clone());
    }
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        let mut i = 0;
        while i < res.len() {
            let current = res.iter().nth(i).cloned().unwrap();
            let dot_symbol = current.next_symbol(rules);
            let next_symbol = current
                .definition(rules)
                .rhs
                .get(current.dot_index + 1)
                .cloned();
            let lookaheads = match next_symbol {
                Some(TerminalOrNonTerminal::Terminal(t)) => {
                    let mut res = BTreeSet::new();
                    res.insert(t);
                    res
                }
                Some(TerminalOrNonTerminal::NonTerminal(nt)) => first.get(&nt).cloned().unwrap(),
                None => {
                    let mut res = BTreeSet::new();
                    res.insert(current.lookahead);
                    res
                }
            };
            if let Some(TerminalOrNonTerminal::NonTerminal(nt)) = dot_symbol {
                (0..rules.len()).for_each(|index| {
                    let rule = &rules[index];
                    if rule.lhs == nt {
                        for lookahead in &lookaheads {
                            let added = res.insert(LR1Item::new(index, 0, *lookahead));
                            if added {
                                has_changed = true;
                            }
                        }
                    }
                });
            }
            i += 1;
        }
    }
    res
}
