use crate::token::TokenType;

use super::TokenLiteralType;

pub trait Expr {
    fn accept<T: NodesVisitor>(&self, visitor: T);
}

pub struct Binary<T: Expr> {
    right: T,
    left: T,
    operation: TokenType
}

impl<T: Expr> Expr for Binary<T> {
    fn accept<V: NodesVisitor>(&self, visitor: V) {
        visitor.doForBinary(&self);
    }
}

impl<T: Expr> Binary<T>{
    pub fn new(right: T, left: T, operation: TokenType) -> Binary<T> {
        Binary {
            right,
            left,
            operation
        }
    }
}

pub struct Unary<T: Expr> {
    right: T,
    operation: TokenType
}

impl<T: Expr> Expr for Unary<T> {
    fn accept<V: NodesVisitor>(&self, visitor: V) {
        visitor.doForUnary(&self);
    }
}

impl<T: Expr> Unary<T> {
    fn new(right: T, operation: TokenType) -> Unary<T> {
        Unary { right, operation }
    }
}

pub struct Literal {
    value: TokenLiteralType
}

impl Expr for Literal {
    fn accept<V: NodesVisitor>(&self, visitor: V) {
        visitor.doForLiteral(&self);
    }
}

pub struct Grouping<T: Expr> {
    expression: T
}

impl<T: Expr> Expr for Grouping<T> {
    fn accept<V: NodesVisitor>(&self, visitor: V) {
        visitor.doForGrouping(&self);
    }
}

impl<T: Expr> Grouping<T> {
    fn new(expression: T) -> Grouping<T> {
        Grouping { expression }
    }
}

pub trait NodesVisitor {
    fn doForGrouping(&self, g: &Grouping<impl Expr>);
    fn doForLiteral(&self, l: &Literal);
    fn doForUnary(&self, u: &Unary<impl Expr>);
    fn doForBinary(&self, b: &Binary<impl Expr>);
}
