use std::collections::BTreeMap;

use super::{
    lr1state::{generate_lr1_statemachine, LR1StateMachine},
    nonterminal::NonTerminalTrait,
    rule::Rule,
    terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
}

pub type ActionTable<Terminal> = BTreeMap<(usize, Terminal), Action>;
pub type GoToTable<NonTerminal> = BTreeMap<(usize, NonTerminal), usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsingTable<
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
    action: ActionTable<Terminal>,
    goto: GoToTable<NonTerminal>,
}

pub fn generate_parsing_table<
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
    precomputed_state_machine: Option<&LR1StateMachine<Terminal, NonTerminal>>,
) -> ParsingTable<Terminal, NonTerminal> {
    let mut res = ParsingTable {
        action: BTreeMap::new(),
        goto: BTreeMap::new(),
    };
    let state_machine = match precomputed_state_machine {
        Some(sm) => sm.clone(),
        None => generate_lr1_statemachine(rules),
    };
    for (state_index, state) in state_machine {
        for (t_or_nt, next_index) in state.transitions {
            match t_or_nt {
                TerminalOrNonTerminal::Terminal(t) => {
                    res.action
                        .insert((state_index, t), Action::Shift(next_index));
                }
                TerminalOrNonTerminal::NonTerminal(nt) => {
                    res.goto.insert((state_index, nt), next_index);
                }
            }
        }
        for item in state.items {
            if item.next_symbol(rules).is_none() {
                if Terminal::is_eof(&item.lookahead) && item.index == 0 {
                    res.action
                        .insert((state_index, Terminal::eof()), Action::Accept);
                } else {
                    res.action
                        .insert((state_index, item.lookahead), Action::Reduce(item.index));
                }
            }
        }
    }
    res
}
