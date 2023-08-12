use std::rc::Rc;
use crate::expression::Expr;
use crate::token::Token;

pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },

    Expression {
        expression: Box<Expr>,
    },

    Function {
        ptr: Rc<FunctionObject>
    },

    If {
        expression: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },

    Print {
        expression: Box<Expr>,
    },

    Var {
        name: Token,
        initializer: Box<Expr>,
    },

    While {
        expression: Box<Expr>,
        body: Box<Stmt>,
    },
}

pub struct FunctionObject {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>
}