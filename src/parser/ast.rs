#[derive(Debug)]
pub enum Expr {
    Equality(Equality),
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
    Identifier(String),
    True,
    False,
    Grouping(Expr),
}
