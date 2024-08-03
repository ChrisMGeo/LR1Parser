use std::marker::PhantomData;

use super::{
    nonterminal::NonTerminalTrait, rule::Rule, terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LR1Item<
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
    pub index: usize,
    pub dot_index: usize,
    pub lookahead: Terminal,
    phantom: PhantomData<NonTerminal>,
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
    > LR1Item<Terminal, NonTerminal>
{
    pub fn new(index: usize, dot_index: usize, lookahead: Terminal) -> Self {
        Self {
            index,
            dot_index,
            lookahead,
            phantom: PhantomData,
        }
    }
    pub fn next_symbol(
        &self,
        rules: &[Rule<Terminal, NonTerminal>],
    ) -> Option<TerminalOrNonTerminal<Terminal, NonTerminal>> {
        rules[self.index].rhs.get(self.dot_index).cloned()
    }
    #[allow(dead_code)]
    pub fn prev_symbol(
        &self,
        rules: &[Rule<Terminal, NonTerminal>],
    ) -> Option<TerminalOrNonTerminal<Terminal, NonTerminal>> {
        if self.dot_index == 0 {
            None
        } else {
            Some(rules[self.index].rhs[self.dot_index - 1])
        }
    }
    pub fn definition(&self, rules: &[Rule<Terminal, NonTerminal>]) -> Rule<Terminal, NonTerminal> {
        rules[self.index].clone()
    }
    pub fn format(&self, rules: &[Rule<Terminal, NonTerminal>]) -> String {
        let def = self.definition(rules);
        let rhs = def.rhs.clone();
        let rhs_mapped: Vec<String> = rhs
            .iter()
            .enumerate()
            .map(|(index, s)| {
                let mut res = "".to_string();
                if index == self.dot_index {
                    res = format!("• {}", res);
                }
                res = format!(
                    "{}{}",
                    res,
                    match s {
                        TerminalOrNonTerminal::Terminal(t) => format!("{:?}", t),
                        TerminalOrNonTerminal::NonTerminal(nt) => format!("{:?}", nt),
                    }
                );
                res
            })
            .collect();
        let mut rhs_str = rhs_mapped.join(" ");
        if self.dot_index == rhs.len() {
            rhs_str.push_str(" •");
        }
        format!("{:?} -> {}, {:?}", def.lhs, rhs_str, self.lookahead)
    }
}

pub fn format_lr1_set<
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
    item_set: &[LR1Item<Terminal, NonTerminal>],
) -> String {
    let items: Vec<String> = item_set
        .iter()
        .map(|item: &LR1Item<Terminal, NonTerminal>| item.format(rules))
        .collect();
    let mut res = items.join("\n\t");
    res.insert(0, '\t');
    res.insert(0, '\n');
    res.insert(0, '{');
    res.push('\n');
    res.push('}');
    res
}
