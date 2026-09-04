#![allow(unused)]

// builder.rs - Builder pattern implementation

use crate::token::{Operator, Token};

#[derive(Debug, Clone)]
pub struct TokenExpression {
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct ExpressionBuilder {
    tokens: Vec<Token>,
    paren_count: i32, // Track parentheses balance
}

impl Default for ExpressionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpressionBuilder {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            paren_count: 0,
        }
    }

    // Add a number to the expression
    pub fn number(mut self, value: f64) -> Self {
        self.tokens.push(Token::number(value));
        self
    }

    // Add an operator
    pub fn operator(mut self, op: Operator) -> Self {
        self.tokens.push(Token::operator(op));
        self
    }

    // Add a variable
    pub fn variable(mut self, name: impl Into<String>) -> Self {
        self.tokens.push(Token::variable(name));
        self
    }

    // Open a parenthesis group
    pub fn open_paren(mut self) -> Self {
        self.tokens.push(Token::OpenParen);
        self.paren_count += 1;
        self
    }

    // Close a parenthesis group
    pub fn close_paren(mut self) -> Result<Self, String> {
        if self.paren_count <= 0 {
            return Err("Unmatched closing parenthesis".to_string());
        }
        self.tokens.push(Token::CloseParen);
        self.paren_count -= 1;
        Ok(self)
    }

    // Build the final expression
    pub fn build(self) -> Result<TokenExpression, String> {
        if self.paren_count != 0 {
            return Err("Unmatched parentheses".to_string());
        }

        if self.tokens.is_empty() {
            return Err("Empty expression".to_string());
        }

        // Validate the expression structure
        self.validate_expression()?;

        Ok(TokenExpression {
            tokens: self.tokens,
        })
    }

    fn validate_expression(&self) -> Result<(), String> {
        // This is a simplistic validation - in a real calculator
        // this would be much more thorough

        if self.tokens.is_empty() {
            return Err("Expression cannot be empty".to_string());
        }

        // Make sure we don't have consecutive operators
        let mut prev_is_op = false;

        for token in &self.tokens {
            match token {
                Token::Operator(_) => {
                    if prev_is_op {
                        return Err("Consecutive operators not allowed".to_string());
                    }
                    prev_is_op = true;
                }
                _ => prev_is_op = false,
            }
        }

        Ok(())
    }
}

// Additional builder methods for common expression patterns
impl ExpressionBuilder {
    // Binary operation (like "2 + 3")
    pub fn binary_op(self, left: f64, op: Operator, right: f64) -> Self {
        self.number(left).operator(op).number(right)
    }

    // Function application (like "sin(x)")
    pub fn function_call(self, func: crate::token::Function, arg: impl Into<String>) -> Self {
        self.function(func)
            .open_paren()
            .variable(arg)
            .close_paren()
            .unwrap() // Safe because we're matching parens
    }

    fn function(mut self, func: crate::token::Function) -> Self {
        self.tokens.push(Token::function(func));
        self
    }
}

// Template methods for common expressions
impl TokenExpression {
    pub fn quadratic() -> ExpressionBuilder {
        ExpressionBuilder::new()
            .number(1.0) // Default a coefficient
            .operator(Operator::Multiply)
            .variable("x")
            .operator(Operator::Power)
            .number(2.0)
            .operator(Operator::Add)
            .number(0.0) // Default b coefficient
            .operator(Operator::Multiply)
            .variable("x")
            .operator(Operator::Add)
            .number(0.0) // Default c coefficient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_number() {
        let expr = ExpressionBuilder::new().number(42.0).build().unwrap();
        assert_eq!(expr.tokens.len(), 1);
    }

    #[test]
    fn test_builder_operator() {
        let expr = ExpressionBuilder::new()
            .number(1.0)
            .operator(Operator::Add)
            .number(2.0)
            .build()
            .unwrap();
        assert_eq!(expr.tokens.len(), 3);
    }

    #[test]
    fn test_builder_variable() {
        let expr = ExpressionBuilder::new()
            .variable("x")
            .build()
            .unwrap();
        assert_eq!(expr.tokens.len(), 1);
    }

    #[test]
    fn test_builder_binary_op() {
        let expr = ExpressionBuilder::new()
            .binary_op(1.0, Operator::Multiply, 2.0)
            .build()
            .unwrap();
        assert_eq!(expr.tokens.len(), 3);
    }

    #[test]
    fn test_builder_parentheses() {
        let expr = ExpressionBuilder::new()
            .open_paren()
            .number(1.0)
            .operator(Operator::Add)
            .number(2.0)
            .close_paren()
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(expr.tokens.len(), 5);
    }

    #[test]
    fn test_builder_unmatched_close_paren() {
        let result = ExpressionBuilder::new().close_paren();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Unmatched closing parenthesis"
        );
    }

    #[test]
    fn test_builder_unmatched_open_parens() {
        let result = ExpressionBuilder::new()
            .open_paren()
            .number(1.0)
            .build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unmatched parentheses");
    }

    #[test]
    fn test_builder_empty_expression() {
        let result = ExpressionBuilder::new().build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Empty expression");
    }

    #[test]
    fn test_builder_consecutive_operators() {
        let result = ExpressionBuilder::new()
            .number(1.0)
            .operator(Operator::Add)
            .operator(Operator::Multiply)
            .number(2.0)
            .build();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Consecutive operators not allowed"
        );
    }

    #[test]
    fn test_builder_function_call() {
        let expr = ExpressionBuilder::new()
            .function_call(crate::token::Function::Sin, "x")
            .build()
            .unwrap();
        assert_eq!(expr.tokens.len(), 4); // sin, (, x, )
    }

    #[test]
    fn test_template_quadratic() {
        let expr = TokenExpression::quadratic().build().unwrap();
        assert!(!expr.tokens.is_empty());
    }

    #[test]
    fn test_builder_chain() {
        let expr = ExpressionBuilder::new()
            .number(2.0)
            .operator(Operator::Add)
            .number(3.0)
            .operator(Operator::Multiply)
            .number(4.0)
            .build()
            .unwrap();
        assert_eq!(expr.tokens.len(), 5);
    }
}
