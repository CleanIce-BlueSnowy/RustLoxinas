use crate::compiler::{
    ast::{
        Expr,
        ExprBinary,
        ExprType,
        ExprUnary,
        ExprVariable,
        Operator,
    },
    token::TokenType,
    parser::{ParseError, Parser},
};
use crate::location::Location;

impl<'a> Parser<'a> {
    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.factor()?;

        loop {
            let token = self.lexer.peek();
            if let TokenType::OpePlus | TokenType::OpeSub = &token.token_type {
                let ope = match self.lexer.advance()?.token_type {
                    TokenType::OpePlus => Operator::Add,
                    TokenType::OpeSub => Operator::Minus,
                    _ => unreachable!(),
                };
                let rhs = self.factor()?;
                lhs = Expr {
                    location: Location::bind(lhs.location.clone(), rhs.location.clone()),
                    expr_type: Box::new(ExprType::Binary(
                        ExprBinary {
                            ope,
                            lhs,
                            rhs,
                        }
                    ))
                };
            } else {
                break Ok(lhs);
            }
        }
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.unary()?;

        loop {
            let token = self.lexer.peek();
            if let TokenType::OpeStar | TokenType::OpeSlash = &token.token_type {
                let ope = match self.lexer.advance()?.token_type {
                    TokenType::OpeStar => Operator::Multi,
                    TokenType::OpeSlash => Operator::Divide,
                    _ => unreachable!(),
                };
                let rhs = self.unary()?;
                lhs = Expr {
                    location: Location::bind(lhs.location.clone(), rhs.location.clone()),
                    expr_type: Box::new(ExprType::Binary(
                        ExprBinary {
                            ope,
                            lhs,
                            rhs,
                        }
                    ))
                };
            } else {
                break Ok(lhs);
            }
        }
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        let token = self.lexer.peek();
        if let TokenType::OpeSub = &token.token_type {
            let start_loc = token.location.clone();
            self.lexer.advance()?;
            let rhs = self.primary()?;
            Ok(Expr {
                location: Location::bind(start_loc, rhs.location.clone()),
                expr_type: Box::new(ExprType::Unary(
                    ExprUnary {
                        ope: Operator::Negative,
                        rhs,
                    }
                ))
            })
        } else if let TokenType::OpePlus = &token.token_type {
            self.lexer.advance()?;
            self.primary()
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.lexer.advance()?;
        if let TokenType::Identifier(name) = &token.token_type {
            Ok(Expr {
                location: token.location.clone(),
                expr_type: Box::new(ExprType::Variable(
                    ExprVariable {
                        name: name.clone(),
                    }
                ))
            })
        } else {
            Err(ParseError {
                location: token.location.clone(),
                msg: "Expect expression.".to_string(),
            })
        }
    }
}
