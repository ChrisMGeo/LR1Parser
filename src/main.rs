use parser_utils::first::compute_firsts;

use crate::parser_types::{
    action_goto::{generate_parsing_table, print_parsing_table},
    augmented_grammar::AugmentedGrammar,
    lr1state::{format_lr1_state_machine, generate_lr1_statemachine},
    nonterminal::NonTerminalTrait,
    rule::Rule,
    terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

pub mod debug;
mod parser_types;
mod parser_utils;

#[derive(Copy, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub enum Terminal {
    EOF,
    LeftParen,
    Name,
    Int,
    RightParen,
    Plus,
    Asterisk,
}
impl std::fmt::Debug for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Asterisk => write!(f, "*"),
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::Name => write!(f, "name"),
            Self::Int => write!(f, "int"),
            Self::EOF => write!(f, "$"),
        }
    }
}
impl TerminalTrait for Terminal {
    fn is_eof(&self) -> bool {
        self == &Terminal::EOF
    }

    fn eof() -> Self {
        Terminal::EOF
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub enum NonTerminal {
    Start,
    Add,
    Factor,
    Term,
}
impl NonTerminalTrait for NonTerminal {
    fn is_start(&self) -> bool {
        self == &NonTerminal::Start
    }

    fn start() -> Self {
        NonTerminal::Start
    }
}

fn main() {
    let augmented_grammar = AugmentedGrammar {
        start_rule: Rule {
            lhs: NonTerminal::Start,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Add)],
        },
        rules: vec![
            Rule {
                lhs: NonTerminal::Add,
                rhs: vec![
                    TerminalOrNonTerminal::NonTerminal(NonTerminal::Add),
                    TerminalOrNonTerminal::Terminal(Terminal::Plus),
                    TerminalOrNonTerminal::NonTerminal(NonTerminal::Factor),
                ],
            },
            Rule {
                lhs: NonTerminal::Add,
                rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Factor)],
            },
            Rule {
                lhs: NonTerminal::Factor,
                rhs: vec![
                    TerminalOrNonTerminal::NonTerminal(NonTerminal::Factor),
                    TerminalOrNonTerminal::Terminal(Terminal::Asterisk),
                    TerminalOrNonTerminal::NonTerminal(NonTerminal::Term),
                ],
            },
            Rule {
                lhs: NonTerminal::Factor,
                rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Term)],
            },
            Rule {
                lhs: NonTerminal::Term,
                rhs: vec![
                    TerminalOrNonTerminal::Terminal(Terminal::LeftParen),
                    TerminalOrNonTerminal::NonTerminal(NonTerminal::Add),
                    TerminalOrNonTerminal::Terminal(Terminal::RightParen),
                ],
            },
            Rule {
                lhs: NonTerminal::Term,
                rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Name)],
            },
            Rule {
                lhs: NonTerminal::Term,
                rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Int)],
            },
        ],
    };
    let final_rules = augmented_grammar.rules();
    debug_println!("Final Rules: {:#?}", final_rules);
    let firsts = compute_firsts(&final_rules);
    debug_println!("{:#?}", firsts);
    // println!("{:#?}", final_rules);
    let lr1_state_machine = generate_lr1_statemachine(&final_rules);
    debug_println!(
        "{}",
        format_lr1_state_machine(&final_rules, &lr1_state_machine)
    );
    let parsing_table = generate_parsing_table(&final_rules, Some(&lr1_state_machine));
    print_parsing_table(&parsing_table, 8);
}
