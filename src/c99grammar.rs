use crate::parser_types::{
    nonterminal::NonTerminalTrait, rule::Rule, terminal::TerminalTrait,
    terminal_or_nonterminal::TerminalOrNonTerminal,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub enum Terminal {
    EOF,
    Identifier,
    Literal,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    SizeOf,
    Typedef,
    Dot,
    Double,
    Float,
    Long,
    Arrow,
    Unsigned,
    Signed,
    Increment,
    Decrement,
    BitwiseAndEquals,
    Comma,
    Extern,
    Semicolon,
    Colon,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Equals,
    EqualsEquals,
    NotEquals,
    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    BitwiseAnd,
    Union,
    Struct,
    Enum,
    Multiply,
    Char,
    Short,
    Int,
    Plus,
    Auto,
    Register,
    Static,
    Minus,
    Const,
    Restrict,
    Tilde,
    Volatile,
    Inline,
    Not,
    Case,
    DefaultCase,
    Divide,
    If,
    Else,
    Ellipsis,
    Percent,
    ExclusiveOrEquals,
    PercentEquals,
    LeftShift,
    Switch,
    Imaginary,
    Complex,
    Bool,
    RightShift,
    While,
    Void,
    Caret,
    ModEquals,
    LeftShiftEquals,
    Pipe,
    For,
    Do,
    Goto,
    LogicalAnd,
    LogicalOr,
    RightShiftEquals,
    QuestionMark,
    BitwiseOrEquals,
    Continue,
    Break,
    Return,
}

impl TerminalTrait for Terminal {
    fn is_eof(&self) -> bool {
        self == &Terminal::EOF
    }
    fn eof() -> Self {
        Terminal::EOF
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub enum NonTerminal {
    Start,
    PrimaryExpression,
    PostfixExpression,
    ArgumentExpressionList,
    UnaryExpression,
    Expression,
    TypeQualifierList,
    DirectDeclarator,
    Pointer,
    ParameterTypeList,
    ParameterList,
    IdentifierList,
    ParameterDeclaration,
    Enumerator,
    StructDeclarator,
    StructDeclarationList,
    StructOrUnion,
    EnumeratorList,
    StructDeclaration,
    StructDeclaratorList,
    SpecifierQualifierList,
    EnumSpecifier,
    AbstractDeclarator,
    Designation,
    BlockItemList,
    JumpStatement,
    IterationStatement,
    BlockItem,
    ExternalDeclaration,
    TranslationUnit,
    DirectAbstractDeclarator,
    SelectionStatement,
    CompoundStatement,
    DesignatorList,
    ExpressionStatement,
    Designator,
    LabeledStatement,
    Statement,
    FunctionDefinition,
    StructOrUnionSpecifier,
    Declarator,
    ShiftOperator,
    ShiftExpression,
    AdditiveOperator,
    RelationalExpression,
    EqualityExpression,
    RelationalOperator,
    EqualityOperator,
    AdditiveExpression,
    MultiplicativeExpression,
    InitializerList,
    TypeName,
    MultiplicativeOperator,
    AssignmentExpression,
    CastExpression,
    UnaryOperator,
    Initializer,
    AndExpression,
    InclusiveOrExpression,
    TypeSpecifier,
    StorageClassSpecifier,
    InitDeclaratorList,
    TypeQualifier,
    InitDeclarator,
    FunctionSpecifier,
    ExclusiveOrExpression,
    DeclarationSpecifiers,
    ConstantExpression,
    LogicalAndExpression,
    Declaration,
    LogicalOrExpression,
    AssignmentOperator,
    ConditionalExpression,
    DeclarationList,
}
impl NonTerminalTrait for NonTerminal {
    fn is_start(&self) -> bool {
        self == &NonTerminal::Start
    }
    fn start() -> Self {
        NonTerminal::Start
    }
}

pub fn c99rules() -> Vec<Rule<Terminal, NonTerminal>> {
    vec![
        Rule {
            lhs: NonTerminal::PrimaryExpression,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Identifier)],
        },
        Rule {
            lhs: NonTerminal::PrimaryExpression,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Literal)],
        },
        Rule {
            lhs: NonTerminal::PrimaryExpression,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::PrimaryExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::PostfixExpression),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::PostfixExpression),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::PostfixExpression),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ArgumentExpressionList),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::PostfixExpression),
                TerminalOrNonTerminal::Terminal(Terminal::Dot),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
            ],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::PostfixExpression),
                TerminalOrNonTerminal::Terminal(Terminal::Arrow),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
            ],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::PostfixExpression),
                TerminalOrNonTerminal::Terminal(Terminal::Increment),
            ],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::PostfixExpression),
                TerminalOrNonTerminal::Terminal(Terminal::Decrement),
            ],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeName),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InitializerList),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::PostfixExpression,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeName),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InitializerList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::ArgumentExpressionList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::AssignmentExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::ArgumentExpressionList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ArgumentExpressionList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::UnaryExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::PostfixExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::UnaryExpression,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Increment),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::UnaryExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::UnaryExpression,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Decrement),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::UnaryExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::UnaryExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::UnaryOperator),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::CastExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::UnaryExpression,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::SizeOf),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::UnaryExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::UnaryExpression,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::SizeOf),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeName),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::UnaryOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::BitwiseAnd)],
        },
        Rule {
            lhs: NonTerminal::UnaryOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Multiply)],
        },
        Rule {
            lhs: NonTerminal::UnaryOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Plus)],
        },
        Rule {
            lhs: NonTerminal::UnaryOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Minus)],
        },
        Rule {
            lhs: NonTerminal::UnaryOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Tilde)],
        },
        Rule {
            lhs: NonTerminal::UnaryOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Not)],
        },
        Rule {
            lhs: NonTerminal::CastExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::UnaryExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::CastExpression,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeName),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::CastExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::MultiplicativeExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::CastExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::MultiplicativeExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::MultiplicativeExpression),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::MultiplicativeOperator),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::CastExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::MultiplicativeOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Multiply)],
        },
        Rule {
            lhs: NonTerminal::MultiplicativeOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Divide)],
        },
        Rule {
            lhs: NonTerminal::MultiplicativeOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Percent)],
        },
        Rule {
            lhs: NonTerminal::AdditiveExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::MultiplicativeExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::AdditiveExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AdditiveExpression),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AdditiveOperator),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::MultiplicativeExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::AdditiveOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Plus)],
        },
        Rule {
            lhs: NonTerminal::AdditiveOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Minus)],
        },
        Rule {
            lhs: NonTerminal::ShiftExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::AdditiveExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::ShiftExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ShiftExpression),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ShiftOperator),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AdditiveExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::ShiftOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::LeftShift)],
        },
        Rule {
            lhs: NonTerminal::ShiftOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::RightShift)],
        },
        Rule {
            lhs: NonTerminal::RelationalExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::ShiftExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::RelationalExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::RelationalExpression),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::RelationalOperator),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ShiftExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::RelationalOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::LessThan)],
        },
        Rule {
            lhs: NonTerminal::RelationalOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::GreaterThan)],
        },
        Rule {
            lhs: NonTerminal::RelationalOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::LessThanEquals)],
        },
        Rule {
            lhs: NonTerminal::RelationalOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::GreaterThanEquals)],
        },
        Rule {
            lhs: NonTerminal::EqualityExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::RelationalExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::EqualityExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::EqualityExpression),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::EqualityOperator),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::RelationalExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::EqualityOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::EqualsEquals)],
        },
        Rule {
            lhs: NonTerminal::EqualityOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::NotEquals)],
        },
        Rule {
            lhs: NonTerminal::AndExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::EqualityExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::AndExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AndExpression),
                TerminalOrNonTerminal::Terminal(Terminal::BitwiseAnd),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::EqualityExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::ExclusiveOrExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::AndExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::ExclusiveOrExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ExclusiveOrExpression),
                TerminalOrNonTerminal::Terminal(Terminal::Caret),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AndExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::InclusiveOrExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::ExclusiveOrExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::InclusiveOrExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InclusiveOrExpression),
                TerminalOrNonTerminal::Terminal(Terminal::Pipe),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ExclusiveOrExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::LogicalAndExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::InclusiveOrExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::LogicalAndExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::LogicalAndExpression),
                TerminalOrNonTerminal::Terminal(Terminal::LogicalAnd),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InclusiveOrExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::LogicalOrExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::LogicalAndExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::LogicalOrExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::LogicalOrExpression),
                TerminalOrNonTerminal::Terminal(Terminal::LogicalOr),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::LogicalAndExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::ConditionalExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::LogicalOrExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::ConditionalExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::LogicalOrExpression),
                TerminalOrNonTerminal::Terminal(Terminal::QuestionMark),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::Colon),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ConditionalExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::AssignmentExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::ConditionalExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::AssignmentExpression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::UnaryExpression),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentOperator),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Equals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::MultiplyEquals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::DivideEquals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::ModEquals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::PlusEquals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::MinusEquals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::LeftShiftEquals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::RightShiftEquals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::BitwiseAndEquals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::ExclusiveOrEquals)],
        },
        Rule {
            lhs: NonTerminal::AssignmentOperator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::BitwiseOrEquals)],
        },
        Rule {
            lhs: NonTerminal::Expression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::AssignmentExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::Expression,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::ConstantExpression,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::ConditionalExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::Declaration,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::Declaration,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InitDeclaratorList),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::DeclarationSpecifiers,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::StorageClassSpecifier,
            )],
        },
        Rule {
            lhs: NonTerminal::DeclarationSpecifiers,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StorageClassSpecifier),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
            ],
        },
        Rule {
            lhs: NonTerminal::DeclarationSpecifiers,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::TypeSpecifier,
            )],
        },
        Rule {
            lhs: NonTerminal::DeclarationSpecifiers,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeSpecifier),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
            ],
        },
        Rule {
            lhs: NonTerminal::DeclarationSpecifiers,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::TypeQualifier,
            )],
        },
        Rule {
            lhs: NonTerminal::DeclarationSpecifiers,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifier),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
            ],
        },
        Rule {
            lhs: NonTerminal::DeclarationSpecifiers,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::FunctionSpecifier,
            )],
        },
        Rule {
            lhs: NonTerminal::DeclarationSpecifiers,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::FunctionSpecifier),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
            ],
        },
        Rule {
            lhs: NonTerminal::InitDeclaratorList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::InitDeclarator,
            )],
        },
        Rule {
            lhs: NonTerminal::InitDeclaratorList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InitDeclaratorList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InitDeclarator),
            ],
        },
        Rule {
            lhs: NonTerminal::InitDeclarator,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Declarator)],
        },
        Rule {
            lhs: NonTerminal::InitDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Declarator),
                TerminalOrNonTerminal::Terminal(Terminal::Equals),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Initializer),
            ],
        },
        Rule {
            lhs: NonTerminal::StorageClassSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Typedef)],
        },
        Rule {
            lhs: NonTerminal::StorageClassSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Extern)],
        },
        Rule {
            lhs: NonTerminal::StorageClassSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Static)],
        },
        Rule {
            lhs: NonTerminal::StorageClassSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Auto)],
        },
        Rule {
            lhs: NonTerminal::StorageClassSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Register)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Void)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Char)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Short)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Int)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Long)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Float)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Double)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Signed)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Unsigned)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Bool)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Complex)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Imaginary)],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::StructOrUnionSpecifier,
            )],
        },
        Rule {
            lhs: NonTerminal::TypeSpecifier,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::EnumSpecifier,
            )],
        },
        Rule {
            lhs: NonTerminal::StructOrUnionSpecifier,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructOrUnion),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructDeclarationList),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::StructOrUnionSpecifier,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructOrUnion),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructDeclarationList),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::StructOrUnionSpecifier,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructOrUnion),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
            ],
        },
        Rule {
            lhs: NonTerminal::StructOrUnion,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Struct)],
        },
        Rule {
            lhs: NonTerminal::StructOrUnion,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Union)],
        },
        Rule {
            lhs: NonTerminal::StructDeclarationList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::StructDeclaration,
            )],
        },
        Rule {
            lhs: NonTerminal::StructDeclarationList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructDeclarationList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructDeclaration),
            ],
        },
        Rule {
            lhs: NonTerminal::StructDeclaration,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::SpecifierQualifierList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructDeclaratorList),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::SpecifierQualifierList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeSpecifier),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::SpecifierQualifierList),
            ],
        },
        Rule {
            lhs: NonTerminal::SpecifierQualifierList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::TypeSpecifier,
            )],
        },
        Rule {
            lhs: NonTerminal::SpecifierQualifierList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifier),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::SpecifierQualifierList),
            ],
        },
        Rule {
            lhs: NonTerminal::SpecifierQualifierList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::TypeQualifier,
            )],
        },
        Rule {
            lhs: NonTerminal::StructDeclaratorList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::StructDeclarator,
            )],
        },
        Rule {
            lhs: NonTerminal::StructDeclaratorList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructDeclaratorList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::StructDeclarator),
            ],
        },
        Rule {
            lhs: NonTerminal::StructDeclarator,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Declarator)],
        },
        Rule {
            lhs: NonTerminal::StructDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Colon),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ConstantExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::StructDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Declarator),
                TerminalOrNonTerminal::Terminal(Terminal::Colon),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ConstantExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::EnumSpecifier,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Enum),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::EnumeratorList),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::EnumSpecifier,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Enum),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::EnumeratorList),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::EnumSpecifier,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Enum),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::EnumeratorList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::EnumSpecifier,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Enum),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::EnumeratorList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::EnumSpecifier,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Enum),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
            ],
        },
        Rule {
            lhs: NonTerminal::EnumeratorList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Enumerator)],
        },
        Rule {
            lhs: NonTerminal::EnumeratorList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::EnumeratorList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Enumerator),
            ],
        },
        Rule {
            lhs: NonTerminal::Enumerator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Identifier)],
        },
        Rule {
            lhs: NonTerminal::Enumerator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
                TerminalOrNonTerminal::Terminal(Terminal::Equals),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ConstantExpression),
            ],
        },
        Rule {
            lhs: NonTerminal::TypeQualifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Const)],
        },
        Rule {
            lhs: NonTerminal::TypeQualifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Restrict)],
        },
        Rule {
            lhs: NonTerminal::TypeQualifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Volatile)],
        },
        Rule {
            lhs: NonTerminal::FunctionSpecifier,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Inline)],
        },
        Rule {
            lhs: NonTerminal::Declarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Pointer),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
            ],
        },
        Rule {
            lhs: NonTerminal::Declarator,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::DirectDeclarator,
            )],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Identifier)],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Declarator),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifierList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentExpression),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifierList),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentExpression),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::Terminal(Terminal::Static),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifierList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentExpression),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifierList),
                TerminalOrNonTerminal::Terminal(Terminal::Static),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentExpression),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifierList),
                TerminalOrNonTerminal::Terminal(Terminal::Multiply),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::Terminal(Terminal::Multiply),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ParameterTypeList),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::IdentifierList),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::Pointer,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Multiply)],
        },
        Rule {
            lhs: NonTerminal::Pointer,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Multiply),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifierList),
            ],
        },
        Rule {
            lhs: NonTerminal::Pointer,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Multiply),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Pointer),
            ],
        },
        Rule {
            lhs: NonTerminal::Pointer,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Multiply),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifierList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Pointer),
            ],
        },
        Rule {
            lhs: NonTerminal::TypeQualifierList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::TypeQualifier,
            )],
        },
        Rule {
            lhs: NonTerminal::TypeQualifierList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifierList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TypeQualifier),
            ],
        },
        Rule {
            lhs: NonTerminal::ParameterTypeList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::ParameterList,
            )],
        },
        Rule {
            lhs: NonTerminal::ParameterTypeList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ParameterList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::Terminal(Terminal::Ellipsis),
            ],
        },
        Rule {
            lhs: NonTerminal::ParameterList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::ParameterDeclaration,
            )],
        },
        Rule {
            lhs: NonTerminal::ParameterList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ParameterList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ParameterDeclaration),
            ],
        },
        Rule {
            lhs: NonTerminal::ParameterDeclaration,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Declarator),
            ],
        },
        Rule {
            lhs: NonTerminal::ParameterDeclaration,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AbstractDeclarator),
            ],
        },
        Rule {
            lhs: NonTerminal::ParameterDeclaration,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::DeclarationSpecifiers,
            )],
        },
        Rule {
            lhs: NonTerminal::IdentifierList,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Identifier)],
        },
        Rule {
            lhs: NonTerminal::IdentifierList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::IdentifierList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
            ],
        },
        Rule {
            lhs: NonTerminal::TypeName,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::SpecifierQualifierList,
            )],
        },
        Rule {
            lhs: NonTerminal::TypeName,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::SpecifierQualifierList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AbstractDeclarator),
            ],
        },
        Rule {
            lhs: NonTerminal::AbstractDeclarator,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Pointer)],
        },
        Rule {
            lhs: NonTerminal::AbstractDeclarator,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::DirectAbstractDeclarator,
            )],
        },
        Rule {
            lhs: NonTerminal::AbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Pointer),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectAbstractDeclarator),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AbstractDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentExpression),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectAbstractDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectAbstractDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::AssignmentExpression),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::Terminal(Terminal::Multiply),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectAbstractDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::Terminal(Terminal::Multiply),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ParameterTypeList),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectAbstractDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::DirectAbstractDeclarator,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DirectAbstractDeclarator),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ParameterTypeList),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
            ],
        },
        Rule {
            lhs: NonTerminal::Initializer,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::AssignmentExpression,
            )],
        },
        Rule {
            lhs: NonTerminal::Initializer,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InitializerList),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::Initializer,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InitializerList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::InitializerList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Initializer)],
        },
        Rule {
            lhs: NonTerminal::InitializerList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Designation),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Initializer),
            ],
        },
        Rule {
            lhs: NonTerminal::InitializerList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InitializerList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Initializer),
            ],
        },
        Rule {
            lhs: NonTerminal::InitializerList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::InitializerList),
                TerminalOrNonTerminal::Terminal(Terminal::Comma),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Designation),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Initializer),
            ],
        },
        Rule {
            lhs: NonTerminal::Designation,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DesignatorList),
                TerminalOrNonTerminal::Terminal(Terminal::Equals),
            ],
        },
        Rule {
            lhs: NonTerminal::DesignatorList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Designator)],
        },
        Rule {
            lhs: NonTerminal::DesignatorList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DesignatorList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Designator),
            ],
        },
        Rule {
            lhs: NonTerminal::Designator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftBracket),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ConstantExpression),
                TerminalOrNonTerminal::Terminal(Terminal::RightBracket),
            ],
        },
        Rule {
            lhs: NonTerminal::Designator,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Dot),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
            ],
        },
        Rule {
            lhs: NonTerminal::Statement,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::LabeledStatement,
            )],
        },
        Rule {
            lhs: NonTerminal::Statement,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::CompoundStatement,
            )],
        },
        Rule {
            lhs: NonTerminal::Statement,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::ExpressionStatement,
            )],
        },
        Rule {
            lhs: NonTerminal::Statement,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::SelectionStatement,
            )],
        },
        Rule {
            lhs: NonTerminal::Statement,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::IterationStatement,
            )],
        },
        Rule {
            lhs: NonTerminal::Statement,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::JumpStatement,
            )],
        },
        Rule {
            lhs: NonTerminal::LabeledStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
                TerminalOrNonTerminal::Terminal(Terminal::Colon),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::LabeledStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Case),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ConstantExpression),
                TerminalOrNonTerminal::Terminal(Terminal::Colon),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::LabeledStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::DefaultCase),
                TerminalOrNonTerminal::Terminal(Terminal::Colon),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::CompoundStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::CompoundStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::LeftBrace),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::BlockItemList),
                TerminalOrNonTerminal::Terminal(Terminal::RightBrace),
            ],
        },
        Rule {
            lhs: NonTerminal::BlockItemList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::BlockItem)],
        },
        Rule {
            lhs: NonTerminal::BlockItemList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::BlockItemList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::BlockItem),
            ],
        },
        Rule {
            lhs: NonTerminal::BlockItem,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Declaration)],
        },
        Rule {
            lhs: NonTerminal::BlockItem,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement)],
        },
        Rule {
            lhs: NonTerminal::ExpressionStatement,
            rhs: vec![TerminalOrNonTerminal::Terminal(Terminal::Semicolon)],
        },
        Rule {
            lhs: NonTerminal::ExpressionStatement,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::SelectionStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::If),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::SelectionStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::If),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
                TerminalOrNonTerminal::Terminal(Terminal::Else),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::SelectionStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Switch),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::IterationStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::While),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::IterationStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Do),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
                TerminalOrNonTerminal::Terminal(Terminal::While),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::IterationStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::For),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ExpressionStatement),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ExpressionStatement),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::IterationStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::For),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ExpressionStatement),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ExpressionStatement),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::IterationStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::For),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Declaration),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ExpressionStatement),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::IterationStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::For),
                TerminalOrNonTerminal::Terminal(Terminal::LeftParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Declaration),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ExpressionStatement),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::RightParenthesis),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Statement),
            ],
        },
        Rule {
            lhs: NonTerminal::JumpStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Goto),
                TerminalOrNonTerminal::Terminal(Terminal::Identifier),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::JumpStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Continue),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::JumpStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Break),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::JumpStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Return),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::JumpStatement,
            rhs: vec![
                TerminalOrNonTerminal::Terminal(Terminal::Return),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Expression),
                TerminalOrNonTerminal::Terminal(Terminal::Semicolon),
            ],
        },
        Rule {
            lhs: NonTerminal::TranslationUnit,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::ExternalDeclaration,
            )],
        },
        Rule {
            lhs: NonTerminal::TranslationUnit,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::TranslationUnit),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::ExternalDeclaration),
            ],
        },
        Rule {
            lhs: NonTerminal::ExternalDeclaration,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(
                NonTerminal::FunctionDefinition,
            )],
        },
        Rule {
            lhs: NonTerminal::ExternalDeclaration,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Declaration)],
        },
        Rule {
            lhs: NonTerminal::FunctionDefinition,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Declarator),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::CompoundStatement),
            ],
        },
        Rule {
            lhs: NonTerminal::FunctionDefinition,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationSpecifiers),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Declarator),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::CompoundStatement),
            ],
        },
        Rule {
            lhs: NonTerminal::DeclarationList,
            rhs: vec![TerminalOrNonTerminal::NonTerminal(NonTerminal::Declaration)],
        },
        Rule {
            lhs: NonTerminal::DeclarationList,
            rhs: vec![
                TerminalOrNonTerminal::NonTerminal(NonTerminal::DeclarationList),
                TerminalOrNonTerminal::NonTerminal(NonTerminal::Declaration),
            ],
        },
    ]
}
