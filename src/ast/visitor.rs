mod expr;

use crate::expr::{Expr, BinaryExpr, UnaryExpr, LiteralValue};

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> T;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> T;
    fn visit_literal_value(&self, expr: &LiteralValue) -> T;
}

pub trait ExprAccept<T> {
    fn accept<V: ExprVisitor<T>>(&self, visitor: &V) -> T;
}

impl ExprAccept<String> for Expr {
    fn accept<V: ExprVisitor<String>>(&self, visitor: &V) -> String {
        match self {
            Expr::BinaryExpr(expr) => visitor.visit_binary_expr(expr),
            Expr::UnaryExpr(expr) => visitor.visit_unary_expr(expr),
            Expr::LiteralValue(expr) => visitor.visit_literal_value(expr),
        }
    }
}