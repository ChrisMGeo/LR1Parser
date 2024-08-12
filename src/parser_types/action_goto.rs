use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::{cell::RefCell, collections::BTreeMap, iter::Peekable};

use super::{
    lr1state::{generate_lr1_statemachine, LR1StateMachine},
    nonterminal::NonTerminalTrait,
    rule::Rule,
    terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
}
impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Shift(n) => write!(f, "s{}", n),
            Action::Reduce(n) => write!(f, "r{}", n),
            Action::Accept => write!(f, "acc"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct TwoKeyMap<K1, K2, V>(pub BTreeMap<(K1, K2), V>);
pub type ActionTable<Terminal> = TwoKeyMap<usize, Terminal, Action>;
pub type GoToTable<NonTerminal> = TwoKeyMap<usize, NonTerminal, usize>;

impl<K1, K2, V> Serialize for TwoKeyMap<K1, K2, V>
where
    K1: Serialize + Ord,
    K2: Serialize + Ord,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_map(None)?;
        let mut iter = self.0.iter().map(|((k1, k2), v)| (k1, k2, v)).peekable();
        loop {
            let Some(&(key1, ..)) = iter.peek() else {
                break;
            };
            ser.serialize_key(key1)?;
            let map_iter = MapIter {
                iter: RefCell::new(&mut iter),
                key: key1,
            };
            ser.serialize_value(&map_iter)?;
        }
        ser.end()
    }
}

struct MapIter<'i, 'k, I, K1>
where
    I: Iterator,
{
    iter: RefCell<&'i mut Peekable<I>>,
    key: &'k K1,
}

impl<'k, I, K1, K2, V> Serialize for MapIter<'_, 'k, I, K1>
where
    I: Iterator<Item = (&'k K1, &'k K2, &'k V)>,
    K1: Eq + 'k,
    K2: Serialize + 'k,
    V: Serialize + 'k,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut iter = self.iter.borrow_mut();
        let mut ser = serializer.serialize_map(None)?;
        while let Some((_, k2, v)) = iter.next_if(|&(k, ..)| k == self.key) {
            ser.serialize_entry(k2, v)?;
        }
        ser.end()
    }
}

impl<'de, K1: Deserialize<'de> + Ord + Copy, K2: Deserialize<'de> + Ord, V: Deserialize<'de>>
    Deserialize<'de> for TwoKeyMap<K1, K2, V>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let maps = BTreeMap::<K1, BTreeMap<K2, V>>::deserialize(deserializer)?;
        Ok(TwoKeyMap(
            maps.into_iter()
                .flat_map(|(k1, k2vs)| k2vs.into_iter().map(move |(k2, v)| ((k1, k2), v)))
                .collect(),
        ))
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsingTable<
    Terminal: std::fmt::Debug
        + Serialize
        + TerminalTrait
        + Copy
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + Ord
        + PartialOrd,
    NonTerminal: std::fmt::Debug
        + Serialize
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
        + Serialize
        + TerminalTrait
        + Copy
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + Ord
        + PartialOrd,
    NonTerminal: std::fmt::Debug
        + Serialize
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
        action: TwoKeyMap(BTreeMap::new()),
        goto: TwoKeyMap(BTreeMap::new()),
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
                        .0
                        .insert((state_index, t), Action::Shift(next_index));
                }
                TerminalOrNonTerminal::NonTerminal(nt) => {
                    res.goto.0.insert((state_index, nt), next_index);
                }
            }
        }
        for item in state.items {
            if item.next_symbol(rules).is_none() {
                if Terminal::is_eof(&item.lookahead) && item.index == 0 {
                    res.action
                        .0
                        .insert((state_index, Terminal::eof()), Action::Accept);
                } else {
                    res.action
                        .0
                        .insert((state_index, item.lookahead), Action::Reduce(item.index));
                }
            }
        }
    }
    res
}

pub fn print_parsing_table<
    Terminal: std::fmt::Debug
        + Serialize
        + TerminalTrait
        + Copy
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + Ord
        + PartialOrd,
    NonTerminal: std::fmt::Debug
        + Serialize
        + NonTerminalTrait
        + Copy
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + PartialOrd
        + Ord,
>(
    parsing_table: &ParsingTable<Terminal, NonTerminal>,
    padding: usize,
) {
    let mut actions_width_for_terminal = BTreeMap::new();
    let mut gotos_width = BTreeMap::new();
    let mut num_states = 0;
    for ((state, terminal), action) in &parsing_table.action.0 {
        let action_width = format!("{:?}", action).len();
        let name_width = format!("{:?}", terminal).len();
        let to_put_width = std::cmp::max(action_width, name_width);
        if actions_width_for_terminal.contains_key(terminal) {
            let current_width = actions_width_for_terminal.get(terminal).unwrap();
            if to_put_width > *current_width {
                actions_width_for_terminal.insert(terminal, to_put_width);
            }
        } else {
            actions_width_for_terminal.insert(terminal, to_put_width);
        }
        num_states = std::cmp::max(num_states, *state + 1);
    }
    for ((state, nonterminal), goto) in &parsing_table.goto.0 {
        let goto_width = format!("{:?}", goto).len();
        let name_width = format!("{:?}", nonterminal).len();
        let to_put_width = std::cmp::max(goto_width, name_width);
        if gotos_width.contains_key(nonterminal) {
            let current_width = gotos_width.get(nonterminal).unwrap();
            if to_put_width > *current_width {
                gotos_width.insert(nonterminal, to_put_width);
            }
        } else {
            gotos_width.insert(nonterminal, to_put_width);
        }
        num_states = std::cmp::max(num_states, *state + 1);
    }
    let num_states_width = format!("{}", num_states - 1).len();
    let state_bar_width = std::cmp::max(num_states_width, "State".to_string().len());
    print!("╔");
    let mut header_length_excluding_tl_tr: usize = 0;
    let mut num_gotos: usize = 0;
    header_length_excluding_tl_tr += state_bar_width + padding * 2 + 1;
    for size in actions_width_for_terminal.values() {
        header_length_excluding_tl_tr += size + padding * 2 + 1;
    }
    for size in gotos_width.values() {
        num_gotos += 1;
        header_length_excluding_tl_tr += size + padding * 2;
    }
    header_length_excluding_tl_tr += num_gotos - 1;
    print!("{}", "═".repeat(header_length_excluding_tl_tr));
    println!("╗");
    print!("║");
    print!("{:^1$}", "LR TABLE", header_length_excluding_tl_tr);
    println!("║");
    print!("╠");
    print!("{}", "═".repeat(state_bar_width + padding * 2));
    print!("╦");
    let mut actions_b = vec![];
    for width in actions_width_for_terminal.values() {
        actions_b.push("═".repeat(width + padding * 2));
    }
    print!("{}", actions_b.join("═"));
    print!("╦");

    let mut gotos_b = vec![];
    for width in gotos_width.values() {
        gotos_b.push("═".repeat(width + padding * 2));
    }
    print!("{}", gotos_b.join("═"));
    println!("╣");
    print!("║");
    print!("{:^1$}", "", state_bar_width + padding * 2);
    print!("║");
    let mut actions_width = 0;
    for size in actions_width_for_terminal.values() {
        actions_width += size + padding * 2 + 1;
    }
    actions_width -= 1;
    print!("{:^1$}", "action", actions_width);
    print!("║");
    let mut goto_width = 0;
    for size in gotos_width.values() {
        goto_width += size + padding * 2 + 1;
    }
    goto_width -= 1;
    print!("{:^1$}", "goto", goto_width);
    println!("║");
    print!("║");
    print!("{:^1$}", "State", state_bar_width + padding * 2);
    print!("╠");
    let mut actions_bottom_border = vec![];
    for width in actions_width_for_terminal.values() {
        actions_bottom_border.push("═".repeat(width + padding * 2));
    }
    print!("{}", actions_bottom_border.join("╦"));
    print!("╬");
    let mut gotos_bottom_border = vec![];
    for width in gotos_width.values() {
        gotos_bottom_border.push("═".repeat(width + padding * 2));
    }
    print!("{}", gotos_bottom_border.join("╦"));
    println!("╣");
    print!("║");
    print!("{:^1$}", "", state_bar_width + padding * 2);
    print!("║");
    for (terminal, width) in &actions_width_for_terminal {
        print!("{:^1$}", format!("{:?}", terminal), width + padding * 2);
        print!("║");
    }
    for (nonterminal, width) in &gotos_width {
        print!("{:^1$}", format!("{:?}", nonterminal), width + padding * 2);
        print!("║");
    }
    println!();
    print!("╠");
    print!("{}", "═".repeat(state_bar_width + padding * 2));
    print!("╬");
    for width in actions_width_for_terminal.values() {
        print!("{}", "═".repeat(width + padding * 2));
        print!("╬");
    }
    let mut gotos_b = vec![];
    for width in gotos_width.values() {
        gotos_b.push("═".repeat(width + padding * 2));
    }
    print!("{}", gotos_b.join("╬"));
    println!("╣");
    for state in 0..num_states {
        print!("║");
        print!("{:<1$}", state, state_bar_width + padding * 2);
        print!("║");
        for (terminal, width) in &actions_width_for_terminal {
            let action = parsing_table
                .action
                .0
                .get(&(state, **terminal))
                .map(|x| format!("{:?}", x))
                .unwrap_or("".to_string());
            print!("{:^1$}", action, width + padding * 2);
            print!("║");
        }
        for (nonterminal, width) in &gotos_width {
            let goto = parsing_table
                .goto
                .0
                .get(&(state, **nonterminal))
                .map(|x| format!("{}", x))
                .unwrap_or("".to_string());
            print!("{:^1$}", goto, width + padding * 2);
            print!("║");
        }
        println!();
        if state != num_states - 1 {
            print!("╠");
            print!("{}", "═".repeat(state_bar_width + padding * 2));
            print!("╬");
            for width in actions_width_for_terminal.values() {
                print!("{}", "═".repeat(width + padding * 2));
                print!("╬");
            }
            print!("{}", gotos_b.join("╬"));
            println!("╣");
        }
    }
    print!("╚");
    print!("{}", "═".repeat(state_bar_width + padding * 2));
    print!("╩");
    for width in actions_width_for_terminal.values() {
        print!("{}", "═".repeat(width + padding * 2));
        print!("╩");
    }
    print!("{}", gotos_b.join("╩"));
    println!("╝");
}

pub fn parse<
    Terminal: std::fmt::Debug
        + Serialize
        + TerminalTrait
        + Copy
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + Ord
        + PartialOrd,
    NonTerminal: std::fmt::Debug
        + Serialize
        + NonTerminalTrait
        + Copy
        + Clone
        + PartialEq
        + std::hash::Hash
        + Eq
        + PartialOrd
        + Ord,
>(
    lex_stream: &[Terminal],
    rules: &[Rule<Terminal, NonTerminal>],
    parsing_table: &ParsingTable<Terminal, NonTerminal>,
) {
    let mut lex_stream = lex_stream.to_owned();
    let rules = rules.to_owned();
    let mut state_stack = vec![0];
    let mut parse_stack: Vec<TerminalOrNonTerminal<Terminal, NonTerminal>> = vec![];
    loop {
        println!("{:?}", state_stack);
        println!("{:?}", parse_stack);
        let state = state_stack.last().unwrap();
        let token = lex_stream.first().unwrap();
        let action = parsing_table.action.0.get(&(*state, *token));
        match action {
            Some(action) => match action {
                Action::Shift(n) => {
                    state_stack.push(*n);
                    parse_stack.push(TerminalOrNonTerminal::Terminal(*token));
                    lex_stream.remove(0);
                }
                Action::Reduce(n) => {
                    let rule = rules[*n].clone();
                    let rule_length = rule.rhs.len();
                    let mut popped = vec![];
                    for _ in 0..rule_length {
                        state_stack.pop();
                        popped.push(parse_stack.pop().unwrap());
                    }
                    let state = state_stack.last().unwrap();
                    let goto = parsing_table.goto.0.get(&(*state, rule.lhs));
                    match goto {
                        Some(goto) => {
                            state_stack.push(*goto);
                            parse_stack.push(TerminalOrNonTerminal::NonTerminal(rule.lhs));
                        }
                        None => {
                            panic!("No goto found");
                        }
                    }
                }
                Action::Accept => {
                    break;
                }
            },
            None => {
                panic!("No action found");
            }
        }
    }
}
