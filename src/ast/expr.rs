// Automatically generated AST
pub enum Expr {
    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    LiteralValue(Box<LiteralValue>),
}
pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Expr,
}
pub struct LiteralValue {
    pub number: f64,
    pub string: String,
}

