use crate::expr::*;
use crate::token::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_logical_and()?;
        while let Some(TokenType::Or) = self.peek().map(|t| &t.token_type) {
            self.advance();
            let right = self.parse_logical_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: TokenType::Or,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;
        while let Some(TokenType::And) = self.peek().map(|t| &t.token_type) {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: TokenType::And,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;
        while let Some(op) = self.peek().map(|t| &t.token_type) {
            match op {
                TokenType::EqualEqual | TokenType::BangEqual => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: op.clone(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;
        while let Some(op) = self.peek().map(|t| &t.token_type) {
            match op {
                TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => {
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: op.clone(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;
        while let Some(op) = self.peek().map(|t| &t.token_type) {
            match op {
                TokenType::Plus | TokenType::Minus => {
                    self.advance();
                    let right = self.parse_factor()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: op.clone(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;
        while let Some(op) = self.peek().map(|t| &t.token_type) {
            match op {
                TokenType::Star | TokenType::Slash | TokenType::Percent => {
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: op.clone(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Bang) | Some(TokenType::Minus) | Some(TokenType::Plus) => {
                let operator = self.advance().token_type.clone();
                let right = self.parse_unary()?;
                Ok(Expr::Unary {
                    operator,
                    right: Box::new(right),
                })
            }
            _ => self.parse_tensor_op(),
        }
    }

    fn parse_tensor_op(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        while let Some(op) = self.peek().map(|t| &t.token_type) {
            match op {
                TokenType::TensorMul | TokenType::TensorDot | TokenType::TensorCross => {
                    self.advance();
                    let right = self.parse_primary()?;
                    expr = Expr::TensorOp {
                        left: Box::new(expr),
                        operator: op.clone(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().map(|t| &t.token_type) {
            Some(TokenType::Number) | Some(TokenType::String) | Some(TokenType::True) | Some(TokenType::False) | Some(TokenType::Nil) => {
                Ok(Expr::Literal(self.advance().clone()))
            }
            Some(TokenType::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            Some(TokenType::Identifier) => {
                self.advance();
                if self.peek().map(|t| &t.token_type) == Some(&TokenType::LeftParen) {
                    self.parse_function_call()
                } else {
                    Ok(Expr::Variable(self.previous().clone()))
                }
            }
            Some(TokenType::LeftBracket) => self.parse_tensor_or_matrix_or_vector_literal(),
            Some(TokenType::Sin) | Some(TokenType::Cos) | Some(TokenType::Tan) |
            Some(TokenType::Asin) | Some(TokenType::Acos) | Some(TokenType::Atan) |
            Some(TokenType::Sinh) | Some(TokenType::Cosh) | Some(TokenType::Tanh) |
            Some(TokenType::Log) | Some(TokenType::Exp) | Some(TokenType::Sqrt) => self.parse_trig_function(),
            _ => Err(ParseError::UnexpectedToken(self.peek().cloned().unwrap_or_default(), "Expected expression.".to_string())),
        }
    }

    // ... (other methods remain largely the same)

    fn consume(&mut self, expected: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&expected) {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken(self.peek().cloned().unwrap_or_default(), message.to_string()))
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        self.peek().map_or(false, |t| &t.token_type == token_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().map_or(true, |t| t.token_type == TokenType::EOF)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}