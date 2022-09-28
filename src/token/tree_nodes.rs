use crate::token::TokenType;

use super::TokenLiteralType;

pub trait Expr {
    fn accept<T: Visitor>(&self, visitor: T);
}

pub struct Binary<R: Expr, L: Expr> {
    right: R,
    left: L,
    operation: TokenType
}

impl<R: Expr, L: Expr> Expr for Binary<R, L> {
    fn accept<V: Visitor>(&self, visitor: V) {
        visitor.doForBinary(&self);
    }
}

impl<R: Expr, L: Expr> Binary<R, L>{
    pub fn new(right: R, left: L, operation: TokenType) -> Binary<R, L> {
        Binary {
            right,
            left,
            operation
        }
    }
}

pub struct Unary<R: Expr> {
    right: R,
    operation: TokenType
}

impl<R: Expr> Expr for Unary<R> {
    fn accept<V: Visitor>(&self, visitor: V) {
        visitor.doForUnary(&self);
    }
}

impl<R: Expr> Unary<R> {
    fn new(right: R, operation: TokenType) -> Unary<R> {
        Unary { right, operation }
    }
}

pub struct Literal {
    value: TokenLiteralType
}

impl Expr for Literal {
    fn accept<V: Visitor>(&self, visitor: V) {
        visitor.doForLiteral(&self);
    }
}

pub struct Grouping<T: Expr> {
    expression: T
}

impl<T: Expr> Expr for Grouping<T> {
    fn accept<V: Visitor>(&self, visitor: V) {
        visitor.doForGrouping(&self);
    }
}

impl<T: Expr> Grouping<T> {
    fn new(expression: T) -> Grouping<T> {
        Grouping { expression }
    }
}

pub trait Visitor {
    fn doForGrouping(&self, g: &Grouping<impl Expr>);
    fn doForLiteral(&self, l: &Literal);
    fn doForUnary(&self, u: &Unary<impl Expr>);
    fn doForBinary(&self, b: &Binary<impl Expr, impl Expr>);
}

struct AstPrinter;

impl AstPrinter {
    fn parenthesize<L: Expr, R: Expr>(&self, name: String, exprs: Vec<impl Expr>) {

    }
}

impl Visitor for AstPrinter {
    fn doForBinary(&self, b: &Binary<impl Expr, impl Expr>) {
        
    }

    fn doForLiteral(&self, l: &Literal) {
        
    }

    fn doForGrouping(&self, g: &Grouping<impl Expr>) {
        
    }

    fn doForUnary(&self, u: &Unary<impl Expr>) {
        
    }
}
