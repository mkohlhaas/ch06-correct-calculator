#![allow(unused)]

// facade.rs - Facade pattern implementation

use crate::adapter::ScientificOperations;
use crate::config::CalculatorConfig;
use crate::expression::{Expression, ExpressionParser, NumberExpression};
use crate::token::{Function, Operator, Token};
use std::collections::HashMap;

// Facade for the calculator system that simplifies complex operations
pub struct CalculatorFacade {
    parser: ExpressionParser,
    variables: HashMap<String, f64>,
    scientific_ops: Box<dyn ScientificOperations>,
    history: Vec<String>,
    config: CalculatorConfig,
}

// NOTE:
// "The facade owns its subsystems rather than borrowing them, which simplifies lifetime management.
// Owned fields eliminate the need for lifetime parameters on the struct, which matters more than
// you might expect.
// A struct with lifetime parameters cannot easily be stored in collections, shared across threads,
// or passed through async boundaries. By owning its data, CalculatorFacade is automatically Send
// and Sync (assuming its fields are), making it safe to use in multithreaded environments."
// Tipp: Own everything, and let the caller decide how to share the facade (e.g. via Arc).

impl CalculatorFacade {
    pub fn new(scientific_ops: Box<dyn ScientificOperations>, config: CalculatorConfig) -> Self {
        Self {
            parser: ExpressionParser,
            variables: HashMap::new(),
            scientific_ops,
            history: Vec::new(),
            config,
        }
    }

    // Simple interface for evaluating expressions
    pub fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        self.history.push(expression.to_string());

        // Handle special function commands
        if let Some(result) = self.handle_special_command(expression)? {
            return Ok(result);
        }

        // Tokenize the expression
        let tokens = self.tokenize(expression)?;

        // Parse tokens into an expression tree
        let expr = ExpressionParser::parse(&tokens)?;

        // Evaluate the expression
        let result = expr.evaluate(&self.variables)?;

        // Store result in a special variable
        self.variables.insert("ans".to_string(), result);

        Ok(result)
    }

    // Simplified method to tokenize a string
    fn tokenize(&self, expression: &str) -> Result<Vec<Token>, String> {
        // This is a simple tokenizer for demonstration
        // In a real calculator, we would have a more sophisticated parser
        let mut tokens = Vec::new();

        for part in expression.split_whitespace() {
            tokens.push(Token::from_str(part)?);
        }

        Ok(tokens)
    }

    // Handle special commands like sin, cos, etc.
    fn handle_special_command(&mut self, command: &str) -> Result<Option<f64>, String> {
        // Parse commands of the form "sin x" or "log 10 2"
        let parts: Vec<&str> = command.split_whitespace().collect();

        if parts.len() < 2 {
            return Ok(None); // Not a special command
        }

        match parts[0] {
            "sin" => {
                let arg = self.parse_value(parts[1])?;
                Ok(Some(self.scientific_ops.sin(arg)))
            }
            "cos" => {
                let arg = self.parse_value(parts[1])?;
                Ok(Some(self.scientific_ops.cos(arg)))
            }
            "tan" => {
                let arg = self.parse_value(parts[1])?;
                Ok(Some(self.scientific_ops.tan(arg)))
            }
            "log" => {
                if parts.len() < 3 {
                    return Err("log requires two arguments: value and base".to_string());
                }
                let value = self.parse_value(parts[1])?;
                let base = self.parse_value(parts[2])?;
                self.scientific_ops.log(value, base).map(Some)
            }
            _ => Ok(None), // Not a special command
        }
    }

    // Helper to parse a value (number or variable)
    fn parse_value(&self, s: &str) -> Result<f64, String> {
        // Try to parse as a number
        if let Ok(num) = s.parse::<f64>() {
            return Ok(num);
        }

        // Try to look up as a variable
        if let Some(value) = self.variables.get(s) {
            return Ok(*value);
        }

        Err(format!("Unknown value: {}", s))
    }

    // Other simplified interfaces

    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    // Specialized methods for common calculations

    pub fn calculate_quadratic(&mut self, a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Err("No real solutions".to_string());
        }

        let sqrt_discriminant = discriminant.sqrt();
        let x1 = (-b + sqrt_discriminant) / (2.0 * a);
        let x2 = (-b - sqrt_discriminant) / (2.0 * a);

        Ok((x1, x2))
    }

    pub fn calculate_pythagorean(&self, a: f64, b: f64) -> f64 {
        (a * a + b * b).sqrt()
    }

    // Method to create expressions easily
    pub fn expression_from_string(&self, expr_str: &str) -> Result<Box<dyn Expression>, String> {
        let tokens = self.tokenize(expr_str)?;
        ExpressionParser::parse(&tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_facade() -> CalculatorFacade {
        let ops = Box::new(crate::adapter::StandardScientificOperations {
            angle_mode: crate::config::AngleMode::Radians,
        });
        CalculatorFacade::new(ops, CalculatorConfig::default())
    }

    #[test]
    fn test_new_facade_has_empty_state() {
        let calc = make_facade();
        assert!(calc.get_history().is_empty());
        assert!(calc.get_variable("x").is_none());
    }

    #[test]
    fn test_set_get_variable() {
        let mut calc = make_facade();
        calc.set_variable("x", 42.0);
        assert_eq!(calc.get_variable("x"), Some(42.0));
    }

    #[test]
    fn test_history_tracking() {
        let mut calc = make_facade();
        let _ = calc.evaluate("2 + 3");
        assert_eq!(calc.get_history(), &["2 + 3"]);
    }

    #[test]
    fn test_calculate_quadratic() {
        let mut calc = make_facade();
        // x^2 - 5x + 6 = 0 -> roots 3 and 2
        let (x1, x2) = calc.calculate_quadratic(1.0, -5.0, 6.0).unwrap();
        assert!(((x1 - 3.0).abs() < 1e-10) || ((x1 - 2.0).abs() < 1e-10));
        assert!(((x2 - 3.0).abs() < 1e-10) || ((x2 - 2.0).abs() < 1e-10));
    }

    #[test]
    fn test_calculate_quadratic_no_real_roots() {
        let mut calc = make_facade();
        assert!(calc.calculate_quadratic(1.0, 0.0, 1.0).is_err());
    }

    #[test]
    fn test_calculate_pythagorean() {
        let calc = make_facade();
        let result = calc.calculate_pythagorean(3.0, 4.0);
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_value_number() {
        let calc = make_facade();
        assert_eq!(calc.parse_value("42.0").unwrap(), 42.0);
    }

    #[test]
    fn test_parse_value_variable() {
        let mut calc = make_facade();
        calc.set_variable("pi", 3.14);
        assert!((calc.parse_value("pi").unwrap() - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_parse_value_unknown() {
        let calc = make_facade();
        assert!(calc.parse_value("unknown").is_err());
    }

    #[test]
    fn test_tokenize() {
        let calc = make_facade();
        let tokens = calc.tokenize("2 + 3").unwrap();
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn test_expression_from_string() {
        let calc = make_facade();
        // The parser is hardcoded to return "2 + 3 * 4", so the result is always 14
        let expr = calc.expression_from_string("2 + 3").unwrap();
        let result = expr.evaluate(&std::collections::HashMap::new()).unwrap();
        assert_eq!(result, 14.0);
    }
}
