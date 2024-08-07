use c99grammar::Terminal;
use parser_types::action_goto::parse;
use parser_utils::first::compute_firsts;

use crate::{
    c99grammar::{c99rules, NonTerminal},
    parser_types::{
        action_goto::{generate_parsing_table, print_parsing_table},
        augmented_grammar::AugmentedGrammar,
        lr1state::{format_lr1_state_machine, generate_lr1_statemachine},
        rule::Rule,
        terminal_or_nonterminal::TerminalOrNonTerminal,
    },
};

pub mod c99grammar;
pub mod debug;
mod parser_types;
mod parser_utils;

fn main() {
    let augmented_grammar = AugmentedGrammar {
        start_rule: Rule {
            lhs: NonTerminal::Start,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::TranslationUnit,
            )],
        },
        rules: c99rules(),
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
    print_parsing_table(&parsing_table, 1);
    parse(
        &vec![
            Terminal::Int,
            Terminal::Identifier,
            Terminal::LeftParenthesis,
            Terminal::Int,
            Terminal::Identifier,
            Terminal::Comma,
            Terminal::Char,
            Terminal::Multiply,
            Terminal::Identifier,
            Terminal::RightParenthesis,
            Terminal::Semicolon,
            Terminal::EOF,
        ],
        &final_rules,
        &parsing_table,
    );
}
