#![allow(unused)]

// expression.rs - Composite pattern for expression trees

// The Composite pattern defines a tree where every node, whether leaf or branch, implements the same Expression trait.

// In this example we have:
// 1. Two leaf nodes (NumberExpression, VariableExpression)
// 2. Two composite nodes (BinaryOperation, FunctionCall)

use crate::token::{Function, Operator};
use std::collections::HashMap;
use std::fmt::{self, Display};

// Expression trait defining common behavior
pub trait Expression: Display {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String>;

    // For debugging and visualization
    fn precedence(&self) -> u8 {
        0 // Leaf nodes have lowest precedence by default
    }
}

// Leaf node for number values
#[derive(Debug, Clone)]
pub struct NumberExpression {
    pub value: f64,
}

impl NumberExpression {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Expression for NumberExpression {
    fn evaluate(&self, _variables: &HashMap<String, f64>) -> Result<f64, String> {
        Ok(self.value)
    }
}

impl Display for NumberExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Leaf node for variables
#[derive(Debug, Clone)]
pub struct VariableExpression {
    pub name: String,
}

impl VariableExpression {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Expression for VariableExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        variables
            .get(&self.name)
            .copied()
            .ok_or_else(|| format!("Undefined variable: {}", self.name))
    }
}

impl Display for VariableExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Composite node for binary operations
// We can't derive Debug and Clone because dyn Expression doesn't implement those traits. NOTE: But maybe should!
pub struct BinaryOperation {
    // NOTE: trait objects provide the dynamic dispatch needed for recursive, heterogeneous tree structures
    // Recursive structures in Rust require heap allocation.
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Operator,
}

impl BinaryOperation {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>, operator: Operator) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }
}

// composite node
impl Expression for BinaryOperation {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let left_val = self.left.evaluate(variables)?;
        let right_val = self.right.evaluate(variables)?;

        match self.operator {
            Operator::Add => Ok(left_val + right_val),
            Operator::Subtract => Ok(left_val - right_val),
            Operator::Multiply => Ok(left_val * right_val),
            Operator::Power => Ok(left_val.powf(right_val)),
            Operator::Divide if right_val == 0.0 => Err("Division by zero".to_string()),
            Operator::Divide => Ok(left_val / right_val),
        }
    }

    fn precedence(&self) -> u8 {
        match self.operator {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Power => 3,
        }
    }
}

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left_str = if self.left.precedence() < self.precedence() {
            format!("({})", self.left)
        } else {
            self.left.to_string()
        };

        let right_str = if self.right.precedence() < self.precedence() {
            format!("({})", self.right)
        } else {
            self.right.to_string()
        };

        write!(f, "{} {} {}", left_str, self.operator_symbol(), right_str)
    }
}

impl BinaryOperation {
    fn operator_symbol(&self) -> &'static str {
        match self.operator {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "^",
        }
    }
}

// Manual implementation of Debug for BinaryOperation
impl fmt::Debug for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BinaryOperation {{ left: <expression>, operator: {:?}, right: <expression> }}",
            self.operator
        )
    }
}

// Composite node for function calls
// We can't derive Debug and Clone because dyn Expression doesn't implement those traits
pub struct FunctionCall {
    pub function: Function,
    pub argument: Box<dyn Expression>,
}

impl FunctionCall {
    pub fn new(function: Function, argument: Box<dyn Expression>) -> Self {
        Self { function, argument }
    }
}

// Manual implementation of Debug for FunctionCall
impl fmt::Debug for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FunctionCall {{ function: {:?}, argument: <expression> }}",
            self.function
        )
    }
}

impl Expression for FunctionCall {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let arg_val = self.argument.evaluate(variables)?;

        match self.function {
            Function::Sin => Ok(arg_val.sin()),
            Function::Cos => Ok(arg_val.cos()),
            Function::Tan => {
                if (arg_val - std::f64::consts::PI / 2.0).abs() % std::f64::consts::PI < 1e-10 {
                    Err("Tangent undefined at this value".to_string())
                } else {
                    Ok(arg_val.tan())
                }
            }
            Function::Sqrt => {
                if arg_val < 0.0 {
                    Err("Cannot take square root of negative number".to_string())
                } else {
                    Ok(arg_val.sqrt())
                }
            }
        }
    }

    fn precedence(&self) -> u8 {
        4 // Function calls have highest precedence
    }
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let func_name = match self.function {
            Function::Sin => "sin",
            Function::Cos => "cos",
            Function::Tan => "tan",
            Function::Sqrt => "sqrt",
        };
        write!(f, "{}({})", func_name, self.argument.to_string())
    }
}

// Parser that builds expression trees from tokens
pub struct ExpressionParser;

impl ExpressionParser {
    // Simple recursive descent parser for demonstration
    pub fn parse(tokens: &[crate::token::Token]) -> Result<Box<dyn Expression>, String> {
        if tokens.is_empty() {
            return Err("Empty expression".to_string());
        }

        // This is a simplified parser - in a real calculator we would
        // implement a proper shunting yard algorithm or recursive descent parser

        // For demonstration, we'll build a simple expression tree for "2 + 3 * 4"
        // which should correctly represent operator precedence

        // In a real implementation, we would parse the tokens recursively

        // For this example, we'll just create a hard-coded expression tree
        // that shows the composite pattern in action
        let multiply = Box::new(BinaryOperation::new(
            Box::new(NumberExpression::new(3.0)),
            Box::new(NumberExpression::new(4.0)),
            Operator::Multiply,
        ));

        let add = Box::new(BinaryOperation::new(
            Box::new(NumberExpression::new(2.0)),
            multiply,
            Operator::Add,
        ));

        Ok(add)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_vars() -> HashMap<String, f64> {
        HashMap::new()
    }

    #[test]
    fn test_number_evaluate() {
        let expr = NumberExpression::new(42.0);
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 42.0);
    }

    #[test]
    fn test_number_display() {
        let expr = NumberExpression::new(42.0);
        assert_eq!(format!("{}", expr), "42");
    }

    #[test]
    fn test_variable_evaluate() {
        let expr = VariableExpression::new("x");
        let mut vars = empty_vars();
        vars.insert("x".to_string(), 3.14);
        assert_eq!(expr.evaluate(&vars).unwrap(), 3.14);
    }

    #[test]
    fn test_variable_undefined() {
        let expr = VariableExpression::new("x");
        assert!(expr.evaluate(&empty_vars()).is_err());
    }

    #[test]
    fn test_variable_display() {
        let expr = VariableExpression::new("my_var");
        assert_eq!(format!("{}", expr), "my_var");
    }

    #[test]
    fn test_binary_add() {
        let expr = BinaryOperation::new(
            Box::new(NumberExpression::new(2.0)),
            Box::new(NumberExpression::new(3.0)),
            Operator::Add,
        );
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 5.0);
    }

    #[test]
    fn test_binary_subtract() {
        let expr = BinaryOperation::new(
            Box::new(NumberExpression::new(10.0)),
            Box::new(NumberExpression::new(4.0)),
            Operator::Subtract,
        );
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 6.0);
    }

    #[test]
    fn test_binary_multiply() {
        let expr = BinaryOperation::new(
            Box::new(NumberExpression::new(3.0)),
            Box::new(NumberExpression::new(4.0)),
            Operator::Multiply,
        );
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 12.0);
    }

    #[test]
    fn test_binary_divide() {
        let expr = BinaryOperation::new(
            Box::new(NumberExpression::new(10.0)),
            Box::new(NumberExpression::new(4.0)),
            Operator::Divide,
        );
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 2.5);
    }

    #[test]
    fn test_binary_division_by_zero() {
        let expr = BinaryOperation::new(
            Box::new(NumberExpression::new(1.0)),
            Box::new(NumberExpression::new(0.0)),
            Operator::Divide,
        );
        assert!(expr.evaluate(&empty_vars()).is_err());
    }

    #[test]
    fn test_binary_power() {
        let expr = BinaryOperation::new(
            Box::new(NumberExpression::new(2.0)),
            Box::new(NumberExpression::new(3.0)),
            Operator::Power,
        );
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 8.0);
    }

    #[test]
    fn test_binary_display() {
        let expr = BinaryOperation::new(
            Box::new(NumberExpression::new(2.0)),
            Box::new(NumberExpression::new(3.0)),
            Operator::Add,
        );
        // NumberExpression has precedence 0, which is < Add's 1, so parens are added
        assert_eq!(format!("{}", expr), "(2) + (3)");
    }

    #[test]
    fn test_binary_display_precedence_parens() {
        // (2) + (3) * (4): all leaves get parens because their precedence is 0
        let mul = BinaryOperation::new(
            Box::new(NumberExpression::new(3.0)),
            Box::new(NumberExpression::new(4.0)),
            Operator::Multiply,
        );
        let add = BinaryOperation::new(
            Box::new(NumberExpression::new(2.0)),
            Box::new(mul),
            Operator::Add,
        );
        assert_eq!(format!("{}", add), "(2) + (3) * (4)");
    }

    #[test]
    fn test_binary_precedence() {
        let add = BinaryOperation::new(
            Box::new(NumberExpression::new(1.0)),
            Box::new(NumberExpression::new(2.0)),
            Operator::Add,
        );
        let mul = BinaryOperation::new(
            Box::new(NumberExpression::new(1.0)),
            Box::new(NumberExpression::new(2.0)),
            Operator::Multiply,
        );
        assert!(add.precedence() < mul.precedence());
    }

    #[test]
    fn test_composite_expression() {
        // 2 + 3 * 4 = 14
        let expr = BinaryOperation::new(
            Box::new(NumberExpression::new(2.0)),
            Box::new(BinaryOperation::new(
                Box::new(NumberExpression::new(3.0)),
                Box::new(NumberExpression::new(4.0)),
                Operator::Multiply,
            )),
            Operator::Add,
        );
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 14.0);
        assert_eq!(format!("{}", expr), "(2) + (3) * (4)");
    }

    #[test]
    fn test_nested_composite() {
        // (2 + 3) * 4 = 20
        let expr = BinaryOperation::new(
            Box::new(BinaryOperation::new(
                Box::new(NumberExpression::new(2.0)),
                Box::new(NumberExpression::new(3.0)),
                Operator::Add,
            )),
            Box::new(NumberExpression::new(4.0)),
            Operator::Multiply,
        );
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 20.0);
        assert_eq!(format!("{}", expr), "((2) + (3)) * (4)");
    }

    #[test]
    fn test_function_sin() {
        use std::f64::consts::PI;
        let mut vars = empty_vars();
        vars.insert("x".to_string(), PI / 2.0);
        let expr = FunctionCall::new(
            Function::Sin,
            Box::new(VariableExpression::new("x")),
        );
        assert!((expr.evaluate(&vars).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_function_cos() {
        let mut vars = empty_vars();
        vars.insert("x".to_string(), 0.0);
        let expr = FunctionCall::new(
            Function::Cos,
            Box::new(VariableExpression::new("x")),
        );
        assert!((expr.evaluate(&vars).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_function_sqrt() {
        let expr = FunctionCall::new(
            Function::Sqrt,
            Box::new(NumberExpression::new(9.0)),
        );
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 3.0);
    }

    #[test]
    fn test_function_sqrt_negative() {
        let expr = FunctionCall::new(
            Function::Sqrt,
            Box::new(NumberExpression::new(-1.0)),
        );
        assert!(expr.evaluate(&empty_vars()).is_err());
    }

    #[test]
    fn test_function_display() {
        let expr = FunctionCall::new(
            Function::Sin,
            Box::new(VariableExpression::new("x")),
        );
        assert_eq!(format!("{}", expr), "sin(x)");
    }

    #[test]
    fn test_parser_returns_2_plus_3_times_4() {
        // The parser ignores tokens and returns hardcoded "2 + 3 * 4"
        let dummy_tokens = vec![crate::token::Token::number(1.0)];
        let expr = ExpressionParser::parse(&dummy_tokens).unwrap();
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 14.0);
    }

    #[test]
    fn test_parser_rejects_empty_tokens() {
        assert!(ExpressionParser::parse(&[]).is_err());
    }

    #[test]
    fn test_debug_impl() {
        let expr = BinaryOperation::new(
            Box::new(NumberExpression::new(1.0)),
            Box::new(NumberExpression::new(2.0)),
            Operator::Add,
        );
        let dbg = format!("{:?}", expr);
        assert!(dbg.contains("BinaryOperation"));
        assert!(dbg.contains("Add"));
    }
}
