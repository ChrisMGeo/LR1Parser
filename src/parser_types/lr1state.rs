use std::collections::{BTreeMap, BTreeSet};

use crate::{
    debug_println, parser_types::lr1item::format_lr1_set, parser_utils::closure::lr1_closure,
};

use super::{
    lr1item::{LR0Item, LR1Item},
    nonterminal::NonTerminalTrait,
    rule::Rule,
    terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

#[derive(Clone, PartialEq)]
pub struct LR1State<
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
    pub state_number: usize,
    pub items: BTreeSet<LR1Item<Terminal, NonTerminal>>,
    pub transitions: BTreeMap<TerminalOrNonTerminal<Terminal, NonTerminal>, usize>,
    pub kernel: Vec<LR1Item<Terminal, NonTerminal>>,
}

pub type LR1StateMachine<Terminal, NonTerminal> = BTreeMap<usize, LR1State<Terminal, NonTerminal>>;

pub fn generate_lr1_statemachine<
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
) -> LR1StateMachine<Terminal, NonTerminal> {
    let mut res = BTreeMap::new();
    let mut n = 0;
    let mut i = 0;
    let firsts = crate::parser_utils::first::compute_firsts(rules);
    let kernel = [LR1Item::new(n, 0, Terminal::eof())];
    res.insert(
        n,
        LR1State {
            state_number: n,
            kernel: kernel.to_vec(),
            items: lr1_closure(rules, &kernel, Some(&firsts)),
            transitions: BTreeMap::new(),
        },
    );
    n += 1;
    while i < n {
        let mut state = res.get(&i).cloned().unwrap();
        let mut transitionable = BTreeSet::new();
        for item in &state.items {
            if let Some(t_or_nt) = item.next_symbol(rules) {
                transitionable.insert(t_or_nt);
            }
        }
        // kernels is a map, where every item in transitionable is mapped to a set of every item in
        // state.items that has that item as the next symbol
        let kernels: BTreeMap<
            TerminalOrNonTerminal<Terminal, NonTerminal>,
            BTreeSet<LR1Item<Terminal, NonTerminal>>,
        > = transitionable
            .iter()
            .map(|nt| {
                let mut res = BTreeSet::new();
                for item in &state.items {
                    if let Some(item_nt) = item.next_symbol(rules) {
                        if item_nt == *nt {
                            res.insert(*item);
                        }
                    }
                }
                (*nt, res)
            })
            .collect();
        for (t_or_nt, kernel) in kernels {
            let kernel = kernel
                .iter()
                .map(|item| LR1Item::new(item.index, item.dot_index + 1, item.lookahead))
                .collect::<Vec<_>>();

            let closured = lr1_closure(
                rules,
                &kernel
                    .iter()
                    .map(|item| LR1Item::new(item.index, item.dot_index, item.lookahead))
                    .collect::<Vec<_>>(),
                Some(&firsts),
            );

            if !closured.is_empty() {
                let closure_as_lr0 = closured
                    .iter()
                    .map(|item| LR0Item {
                        index: item.index,
                        dot_index: item.dot_index,
                    })
                    .collect::<BTreeSet<LR0Item>>();
                match res.iter_mut().find(|(_, state)| {
                    state
                        .items
                        .iter()
                        .map(|item| LR0Item {
                            index: item.index,
                            dot_index: item.dot_index,
                        })
                        .collect::<BTreeSet<LR0Item>>()
                        == closure_as_lr0
                }) {
                    Some((target, s)) => {
                        s.items.extend(closured);
                        state.transitions.insert(t_or_nt, *target);
                    }
                    None => {
                        debug_println!(
                            "goto({:?}, {:?}): {} = state: {}: {}",
                            state.state_number,
                            t_or_nt,
                            format_lr1_set(rules, &kernel.to_vec()),
                            n,
                            format_lr1_set(rules, &closured.iter().cloned().collect::<Vec<_>>())
                        );
                        res.insert(
                            n,
                            LR1State {
                                kernel: kernel.to_vec(),
                                state_number: n,
                                items: closured,
                                transitions: BTreeMap::new(),
                            },
                        );
                        state.transitions.insert(t_or_nt, n);
                        n += 1;
                    }
                }
            }
        }
        res.get_mut(&i).unwrap().transitions = state.transitions;
        i += 1;
    }
    res
}

pub fn format_lr1_state_machine<
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
    statemachine: &LR1StateMachine<Terminal, NonTerminal>,
) -> String {
    let mut res = String::new();
    res.push_str("{\n"); // 1
    for (state_number, state) in statemachine {
        res.push_str("\t{\n"); // 2
        res.push_str(&format!("\t\tstate {}:\n\t\t", state_number));
        res.push_str(
            &format_lr1_set(rules, &state.items.iter().cloned().collect::<Vec<_>>())
                .as_str()
                .split('\n')
                .map(|c| format!("\t\t{}", c))
                .collect::<Vec<String>>()
                .join("\n\t\t"),
        );
        res.push_str("\n\t\ttransitions: {\n"); // 3
        for (t_or_nt, target) in &state.transitions {
            res.push_str(&format!("\t\t\t{:?} -> state {},\n", t_or_nt, target));
        }
        res.push_str("\t\t},\n"); // 3
        res.push_str("\n\t},\n"); // 2
    }
    res.push('}'); // 1
    res
}
