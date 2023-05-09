/*
planning out future of ast

program        ->  declaration* EOF

declaration    ->  function | var | statement
function       ->  FUN IDENTIFIER "()" block   // TODO: Can't handle args or multiple funs
var            ->  ( "val" | "var" ) IDENT "=" expression ";"*
statement      ->  expression | for | loop | print | return

// Misc
block          ->  "{" declaration* "}"

// Expressions
expression     ->  assignment ";"*
assignment     ->  IDENT "=" logic_or | logic_or
logic_or       ->  logic_and ( "or" logic_and )*
logic_and      ->  equality ( "and" equality )*
equality       ->  comparison ( ( "!=" | "==" ) comparison )*
comparison     ->  term ( ( ">" | ">=" | "<" | "<=" ) term )*
term           ->  factor ( ( "-" | "+" ) factor )*
factor         ->  unary ( ( "/" | "*" ) unary )*
unary          ->  ( "!" | "-" ) unary | primary
primary        ->  INT | FLOAT | STRING | call | "true" | "false" | "(" expression ")"
call           ->  IDENT"()" | IDENT # TODO: Need to implement function args
 */

// Declarations

#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Declaration {
    Variable(Variable),
    Statement(Statement),
    Function(Function),
}

#[derive(Debug)]
pub struct Function {
    // TODO: Add args
    pub ident: Identifier,
    pub block: Block,
}

#[derive(Debug)]
pub enum VariableType {
    Var,
    Val,
}

#[derive(Debug)]
pub struct Variable {
    pub v_type: VariableType,
    pub ident: Identifier,
    pub value: Expr,
}

#[derive(Debug)]
pub enum Statement {
    // TODO: Add other statement types
    Expression(Expr)
}

// Misc

#[derive(Debug)]
pub struct Block {
    pub declarations: Vec<Declaration>,
}

// Expressions

#[derive(Debug)]
pub enum Expr {
    LogicOr(LogicOr),
}

#[derive(Debug)]
pub enum LogicOrLeft {
    LogicAnd(LogicAnd),
    LogicOr(Box<LogicOr>),
}

#[derive(Debug)]
pub struct LogicOr {
    pub left: LogicOrLeft,
    pub right: Option<LogicAnd>,
}

#[derive(Debug)]
pub enum LogicAndLeft {
    Equality(Equality),
    LogicAnd(Box<LogicAnd>),
}

#[derive(Debug)]
pub struct LogicAnd {
    pub left: LogicAndLeft,
    pub right: Option<Equality>,
}

#[derive(Debug)]
pub enum EqualityOp {
    NotEqual,
    Equal,
}

#[derive(Debug)]
pub struct EqualityRight {
    pub op: EqualityOp,
    pub right: Comparison,
}

#[derive(Debug)]
pub enum EqualityLeft {
    Comparison(Comparison),
    Equality(Box<Equality>)
}

#[derive(Debug)]
pub struct Equality {
    pub left: EqualityLeft,
    pub right: Option<EqualityRight>,
}

#[derive(Debug)]
pub enum ComparisonOp {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
pub struct ComparisonRight {
    pub op: ComparisonOp,
    pub right: Term,
}

#[derive(Debug)]
pub enum ComparisonLeft {
    Term(Term),
    Comparison(Box<Comparison>)
}

#[derive(Debug)]
pub struct Comparison {
    pub left: ComparisonLeft,
    pub right: Option<ComparisonRight>,
}

#[derive(Debug)]
pub enum TermOp {
    Minus,
    Plus,
}

#[derive(Debug)]
pub struct TermRight {
    pub op: TermOp,
    pub right: Factor,
}

#[derive(Debug)]
pub enum TermLeft {
    Factor(Factor),
    Term(Box<Term>)
}

#[derive(Debug)]
pub struct Term {
    pub left: TermLeft,
    pub right: Option<TermRight>,
}

#[derive(Debug)]
pub enum FactorOp {
    Div,
    Mult,
}

#[derive(Debug)]
pub struct FactorRight {
    pub op: FactorOp,
    pub right: Unary,
}

#[derive(Debug)]
pub enum FactorLeft {
    Unary(Unary),
    Factor(Box<Factor>)
}

#[derive(Debug)]
pub struct Factor {
    pub left: FactorLeft,
    pub right: Option<FactorRight>,
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug)]
pub enum UnaryRight {
    Unary(Unary),
    Primary(Primary),
}

#[derive(Debug)]
pub struct Unary {
    pub op: Option<UnaryOp>,
    pub right: Box<UnaryRight>,
}

#[derive(Debug)]
pub enum Primary {
    Int(String),
    Float(String),
    String(String),
    Identifier(Identifier),
    True,
    False,
    Grouping(Expr),
}

#[derive(Debug)]
pub struct Identifier(pub String);
