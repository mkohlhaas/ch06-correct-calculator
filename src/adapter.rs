#![allow(unused)]

// adapter.rs - Adapter pattern implementation

use crate::config::AngleMode;
use crate::expression::Expression;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fmt::{self, Display};

// ============================================= //
// Uniform Interface for Scientific Calculations //
// ============================================= //

pub trait ScientificOperations {
    fn sin(&self, angle: f64) -> f64;
    fn cos(&self, angle: f64) -> f64;
    fn tan(&self, angle: f64) -> f64;
    fn log(&self, value: f64, base: f64) -> Result<f64, String>;
}

// 1. Far-fetched example: StandardScientificOperations and ExternalLibraryAdapter are identical
// 2. Better example would be a StandardScientificOperations using an external library.
// 3. From/Into traits would be cleaner than a full adapter struct for converting degrees and radians.

// Standard implementation using Rust's math functions
pub struct StandardScientificOperations {
    pub angle_mode: AngleMode,
}

// Rust's trigonometric math functions use radians by default
impl ScientificOperations for StandardScientificOperations {
    fn sin(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.sin(),
            AngleMode::Degrees => (angle * PI / 180.0).sin(),
        }
    }

    fn cos(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.cos(),
            AngleMode::Degrees => (angle * PI / 180.0).cos(),
        }
    }

    fn tan(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle.tan(),
            AngleMode::Degrees => (angle * PI / 180.0).tan(),
        }
    }

    fn log(&self, value: f64, base: f64) -> Result<f64, String> {
        if value <= 0.0 {
            return Err("Cannot take logarithm of non-positive number".to_string());
        }

        if base <= 0.0 || base == 1.0 {
            return Err("Invalid logarithm base".to_string());
        }

        Ok((value.ln()) / (base.ln()))
    }
}

// ================================================ //
// Adapter for a hypothetical external math library //
// ================================================ //

pub struct ExternalLibraryAdapter {
    // In a real implementation, this would contain a reference to the external library
    angle_mode: AngleMode,
}

impl ExternalLibraryAdapter {
    pub fn new(angle_mode: AngleMode) -> Self {
        Self { angle_mode }
    }

    // This would be a helper that converts to the format needed by the external library
    // Returns radians
    fn convert_angle(&self, angle: f64) -> f64 {
        match self.angle_mode {
            AngleMode::Radians => angle,
            AngleMode::Degrees => angle * PI / 180.0,
        }
    }
}

impl ScientificOperations for ExternalLibraryAdapter {
    fn sin(&self, angle: f64) -> f64 {
        let converted_angle = self.convert_angle(angle);

        // In a real implementation, we would call the external library's function
        // For this example, we'll just use Rust's built-in function
        converted_angle.sin()
    }

    fn cos(&self, angle: f64) -> f64 {
        let converted_angle = self.convert_angle(angle);
        converted_angle.cos()
    }

    fn tan(&self, angle: f64) -> f64 {
        let converted_angle = self.convert_angle(angle);
        converted_angle.tan()
    }

    fn log(&self, value: f64, base: f64) -> Result<f64, String> {
        // Simulate calling an external library function
        if value <= 0.0 {
            return Err("Cannot take logarithm of non-positive number".to_string());
        }

        if base <= 0.0 || base == 1.0 {
            return Err("Invalid logarithm base".to_string());
        }

        Ok((value.ln()) / (base.ln()))
    }
}

// ============================================== //
// Adapters to connect different expression types //
// ============================================== //

// Adapter for using ScientificOperations with Expression
pub struct ScientificFunctionExpression {
    operation: Box<dyn Fn(f64) -> f64>,
    arg_expression: Box<dyn Expression>,
    description: String,
}

impl ScientificFunctionExpression {
    pub fn new_sin(
        scientific_ops: Box<dyn ScientificOperations>,
        arg_expression: Box<dyn Expression>,
    ) -> Self {
        // We need to move the scientific_ops into the closure
        // This is a bit tricky in Rust without interior mutability
        let operation = Box::new(move |angle: f64| scientific_ops.sin(angle));

        Self {
            operation,
            arg_expression,
            description: "sin".to_string(),
        }
    }

    pub fn new_cos(
        scientific_ops: Box<dyn ScientificOperations>,
        arg_expression: Box<dyn Expression>,
    ) -> Self {
        let operation = Box::new(move |angle: f64| scientific_ops.cos(angle));
        Self {
            operation,
            arg_expression,
            description: "cos".to_string(),
        }
    }

    pub fn new_tan(
        scientific_ops: Box<dyn ScientificOperations>,
        arg_expression: Box<dyn Expression>,
    ) -> Self {
        let operation = Box::new(move |angle: f64| scientific_ops.tan(angle));

        Self {
            operation,
            arg_expression,
            description: "tan".to_string(),
        }
    }
}

impl Expression for ScientificFunctionExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let arg_value = self.arg_expression.evaluate(variables)?;
        Ok((self.operation)(arg_value))
    }

    fn precedence(&self) -> u8 {
        // Function calls have highest precedence
        4
    }
}

impl Display for ScientificFunctionExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.description, self.arg_expression)
    }
}

// Two-way adapter: allows Expression to be used where ScientificOperations is expected
pub struct ExpressionScientificAdapter {
    sin_expr: Box<dyn Expression>,
    cos_expr: Box<dyn Expression>,
    tan_expr: Box<dyn Expression>,
    log_expr: Box<dyn Expression>,
}

impl ExpressionScientificAdapter {
    pub fn new(
        sin_expr: Box<dyn Expression>,
        cos_expr: Box<dyn Expression>,
        tan_expr: Box<dyn Expression>,
        log_expr: Box<dyn Expression>,
    ) -> Self {
        Self {
            sin_expr,
            cos_expr,
            tan_expr,
            log_expr,
        }
    }
}

impl ScientificOperations for ExpressionScientificAdapter {
    fn sin(&self, angle: f64) -> f64 {
        // Create a variables map with the angle as a variable
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), angle);

        // Evaluate the sin expression with this variable
        //  ⚠️ silently swallows error!
        self.sin_expr.evaluate(&variables).unwrap_or(0.0)
    }

    fn cos(&self, angle: f64) -> f64 {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), angle);

        //  ⚠️ silently swallows error!
        self.cos_expr.evaluate(&variables).unwrap_or(0.0)
    }

    fn tan(&self, angle: f64) -> f64 {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), angle);

        //  ⚠️ silently swallows error!
        self.tan_expr.evaluate(&variables).unwrap_or(0.0)
    }

    fn log(&self, value: f64, base: f64) -> Result<f64, String> {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), value);
        variables.insert("base".to_string(), base);

        self.log_expr.evaluate(&variables)
    }
}

// ===== //
// Tests //
// ===== //

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(x: f64) -> HashMap<String, f64> {
        let mut m = HashMap::new();
        m.insert("x".to_string(), x);
        m
    }

    #[test]
    fn test_standard_sin_radians() {
        let ops = StandardScientificOperations {
            angle_mode: AngleMode::Radians,
        };
        assert!((ops.sin(PI / 2.0) - 1.0).abs() < 1e-10);
        assert!((ops.sin(0.0)).abs() < 1e-10);
    }

    #[test]
    fn test_standard_sin_degrees() {
        let ops = StandardScientificOperations {
            angle_mode: AngleMode::Degrees,
        };
        assert!((ops.sin(90.0) - 1.0).abs() < 1e-10);
        assert!((ops.sin(0.0)).abs() < 1e-10);
    }

    #[test]
    fn test_standard_cos_radians() {
        let ops = StandardScientificOperations {
            angle_mode: AngleMode::Radians,
        };
        assert!((ops.cos(0.0) - 1.0).abs() < 1e-10);
        assert!((ops.cos(PI) + 1.0).abs() < 1e-10); // cos(π) ≈ -1
    }

    #[test]
    fn test_standard_cos_degrees() {
        let ops = StandardScientificOperations {
            angle_mode: AngleMode::Degrees,
        };
        assert!((ops.cos(0.0) - 1.0).abs() < 1e-10);
        assert!((ops.cos(180.0) + 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_standard_tan() {
        let ops = StandardScientificOperations {
            angle_mode: AngleMode::Radians,
        };
        assert!((ops.tan(0.0)).abs() < 1e-10);
        assert!((ops.tan(PI / 4.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_standard_log() {
        let ops = StandardScientificOperations {
            angle_mode: AngleMode::Radians,
        };
        let result = ops.log(100.0, 10.0).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_standard_log_invalid_value() {
        let ops = StandardScientificOperations {
            angle_mode: AngleMode::Radians,
        };
        assert!(ops.log(0.0, 10.0).is_err());
        assert!(ops.log(-1.0, 10.0).is_err());
    }

    #[test]
    fn test_standard_log_invalid_base() {
        let ops = StandardScientificOperations {
            angle_mode: AngleMode::Radians,
        };
        assert!(ops.log(10.0, 0.0).is_err());
        assert!(ops.log(10.0, 1.0).is_err());
    }

    #[test]
    fn test_external_adapter_sin_radians() {
        let ops = ExternalLibraryAdapter::new(AngleMode::Radians);
        assert!((ops.sin(PI / 2.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_external_adapter_sin_degrees() {
        let ops = ExternalLibraryAdapter::new(AngleMode::Degrees);
        assert!((ops.sin(90.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_scientific_function_expression() {
        let ops = Box::new(StandardScientificOperations {
            angle_mode: AngleMode::Radians,
        });
        let expr = ScientificFunctionExpression::new_sin(
            ops,
            Box::new(crate::expression::VariableExpression::new("x")),
        );
        let result = expr.evaluate(&vars(PI / 2.0)).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_scientific_function_expression_display() {
        let ops = Box::new(StandardScientificOperations {
            angle_mode: AngleMode::Radians,
        });
        let expr = ScientificFunctionExpression::new_sin(
            ops,
            Box::new(crate::expression::VariableExpression::new("x")),
        );
        assert_eq!(format!("{}", expr), "sin(x)");
    }

    #[test]
    fn test_expression_scientific_adapter() {
        use crate::expression::{BinaryOperation, FunctionCall, NumberExpression};
        use crate::token::{Function, Operator};

        let sin_x = Box::new(FunctionCall::new(
            Function::Sin,
            Box::new(crate::expression::VariableExpression::new("x")),
        ));
        let cos_x = Box::new(FunctionCall::new(
            Function::Cos,
            Box::new(crate::expression::VariableExpression::new("x")),
        ));
        let tan_x = Box::new(FunctionCall::new(
            Function::Tan,
            Box::new(crate::expression::VariableExpression::new("x")),
        ));
        // log(x, base) = ln(x)/ln(base) -- express as a binary op
        let log_x = Box::new(BinaryOperation::new(
            Box::new(FunctionCall::new(
                Function::Sqrt,
                Box::new(crate::expression::NumberExpression::new(100.0)),
            )),
            Box::new(NumberExpression::new(2.0)),
            Operator::Divide,
        ));

        let adapter = ExpressionScientificAdapter::new(sin_x, cos_x, tan_x, log_x);
        assert!((adapter.sin(PI / 2.0) - 1.0).abs() < 1e-10);
        assert!((adapter.cos(0.0) - 1.0).abs() < 1e-10);
        assert!((adapter.tan(0.0)).abs() < 1e-10);
        // log_expr is sqrt(100)/2 = 5.0
        let result = adapter.log(10.0, 2.0).unwrap();
        assert!((result - 5.0).abs() < 1e-10);
    }
}
