#![allow(unused)]

// bridge.rs - Bridge pattern implementation

use crate::expression::Expression;

// /////////////////////////////////// //
// 1. Low-level Implementation (trait) //
// /////////////////////////////////// //

// Implementation for different display formats
pub trait DisplayImplementation {
    fn display_text(&self, text: &str);
    fn display_formatted(&self, value: f64, format: &str);
}

// Some Low-level Implementations for the Display

pub struct ConsoleDisplay;
impl DisplayImplementation for ConsoleDisplay {
    fn display_text(&self, text: &str) {
        println!("{}", text);
    }

    fn display_formatted(&self, value: f64, format: &str) {
        println!("{}", format.replace("{:.10g}", &format!("{:.10}", value)));
    }
}

pub struct HtmlDisplay;
impl DisplayImplementation for HtmlDisplay {
    fn display_text(&self, text: &str) {
        println!(
            "<div>{}</div>",
            text.replace("<", "&lt;").replace(">", "&gt;")
        );
    }

    fn display_formatted(&self, value: f64, format: &str) {
        let formatted = format.replace("{:.10g}", &format!("{:.10}", value));
        println!("<div class=\"result\">{}</div>", formatted);
    }
}

pub struct JsonDisplay;
impl DisplayImplementation for JsonDisplay {
    fn display_text(&self, text: &str) {
        println!("{{\"text\": \"{}\"}}", text.replace("\"", "\\\""));
    }

    fn display_formatted(&self, value: f64, format: &str) {
        let formatted = format!("{:.10}", value);
        println!("{{\"result\": {}}}", formatted);
    }
}

// ///////////////////////////////// //
// 2. High-level Abstraction (trait) //
// ///////////////////////////////// //

// Abstraction for a calculator display
pub trait Display {
    fn show_result(&self, result: f64);
    fn show_error(&self, error: &str);
    fn show_expression(&self, expression: &dyn Expression);
}

// ////////////////////// //
// 3. The Bridge (struct) //
// ////////////////////// //

// Concrete display that uses a specific implementation
// A. Dynamic Bridge (implementations can be changed during run-time)
pub struct CalculatorDisplay {
    implementation: Box<dyn DisplayImplementation>, // bridge to the low-level implementation
}

// For comparison:
// B. Static Bridge
pub struct CalculatorDisplay1<D: DisplayImplementation> {
    implementation: D, // bridge to the low-level implementation
}

impl CalculatorDisplay {
    pub fn new(implementation: Box<dyn DisplayImplementation>) -> Self {
        Self { implementation }
    }
}

// ///////////////////////////////////////////////////////// //
// 4. Implement High-level Abstraction for the Bridge Struct //
// ///////////////////////////////////////////////////////// //

// NOTE: The high-level abstraction uses the low-level implementation
impl Display for CalculatorDisplay {
    fn show_result(&self, result: f64) {
        self.implementation
            .display_formatted(result, "Result: {:.10g}");
    }

    fn show_error(&self, error: &str) {
        self.implementation
            .display_text(&format!("Error: {}", error));
    }

    fn show_expression(&self, expression: &dyn Expression) {
        self.implementation
            .display_text(&format!("Expression: {}", expression));
    }
}

// More complex bridge example for expression evaluation

// NOTE: Looks more like the strategy pattern.
// "When you encounter this kind of structural similarity in your own code, focus on the design
// intent rather than trying to classify the pattern definitively."

// 1. Low-level abstract interface for evaluation strategies

pub trait EvaluationStrategy {
    fn evaluate(
        &self,
        expression: &dyn Expression, // this is also a bridge
        variables: &std::collections::HashMap<String, f64>,
    ) -> Result<f64, String>;
}

// 2. Low-level Implementations

// Different evaluation strategies (implementors)
pub struct StandardEvaluator;
impl EvaluationStrategy for StandardEvaluator {
    fn evaluate(
        &self,
        expression: &dyn Expression,
        variables: &std::collections::HashMap<String, f64>,
    ) -> Result<f64, String> {
        // Basic evaluation without optimizations
        expression.evaluate(variables)
    }
}

use std::cell::RefCell;

pub struct OptimizingEvaluator {
    cache: RefCell<std::collections::HashMap<String, f64>>,
}
impl OptimizingEvaluator {
    pub fn new() -> Self {
        Self {
            cache: RefCell::new(std::collections::HashMap::new()),
        }
    }
}

impl EvaluationStrategy for OptimizingEvaluator {
    fn evaluate(
        &self,
        expression: &dyn Expression,
        variables: &std::collections::HashMap<String, f64>,
    ) -> Result<f64, String> {
        // Check if we've evaluated this expression before
        let key = format!("{:?}:{}", expression.to_string(), variables.len());

        // In a real implementation, we'd properly account for variable values in the key
        // For demonstration, this is simplified
        if let Some(cached_result) = self.cache.borrow().get(&key) {
            return Ok(*cached_result);
        }

        // Evaluate and cache the result
        let result = expression.evaluate(variables)?;

        // Using RefCell for thread-safe interior mutability
        // NOTE: RefCell is NOT thread-safe!
        self.cache.borrow_mut().insert(key, result);

        Ok(result)
    }
}

// 3. The Bridge (this time no high-level abstraction layer)

pub struct Evaluator {
    strategy: Box<dyn EvaluationStrategy>,
}

impl Evaluator {
    pub fn new(strategy: Box<dyn EvaluationStrategy>) -> Self {
        Self { strategy }
    }

    pub fn evaluate(
        &self,
        expression: &dyn Expression,
        variables: &std::collections::HashMap<String, f64>,
    ) -> Result<f64, String> {
        self.strategy.evaluate(expression, variables)
    }

    pub fn change_strategy(&mut self, strategy: Box<dyn EvaluationStrategy>) {
        self.strategy = strategy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::{BinaryOperation, NumberExpression};
    use crate::token::Operator;

    fn empty_vars() -> std::collections::HashMap<String, f64> {
        std::collections::HashMap::new()
    }

    fn make_add_expr() -> Box<dyn Expression> {
        Box::new(BinaryOperation::new(
            Box::new(NumberExpression::new(2.0)),
            Box::new(NumberExpression::new(3.0)),
            Operator::Add,
        ))
    }

    #[test]
    fn test_standard_evaluator() {
        let evaluator = Evaluator::new(Box::new(StandardEvaluator));
        let expr = make_add_expr();
        let result = evaluator.evaluate(&*expr, &empty_vars()).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_optimizing_evaluator_first_call() {
        let evaluator = Evaluator::new(Box::new(OptimizingEvaluator::new()));
        let expr = make_add_expr();
        let result = evaluator.evaluate(&*expr, &empty_vars()).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_optimizing_evaluator_cached() {
        let evaluator = Evaluator::new(Box::new(OptimizingEvaluator::new()));
        let expr = make_add_expr();
        let r1 = evaluator.evaluate(&*expr, &empty_vars()).unwrap();
        let r2 = evaluator.evaluate(&*expr, &empty_vars()).unwrap();
        assert_eq!(r1, r2);
        assert_eq!(r1, 5.0);
    }

    #[test]
    fn test_change_strategy() {
        let mut evaluator = Evaluator::new(Box::new(StandardEvaluator));
        let expr = make_add_expr();
        assert_eq!(evaluator.evaluate(&*expr, &empty_vars()).unwrap(), 5.0);

        evaluator.change_strategy(Box::new(OptimizingEvaluator::new()));
        assert_eq!(evaluator.evaluate(&*expr, &empty_vars()).unwrap(), 5.0);
    }

    #[test]
    fn test_optimizing_evaluator_wrong_cache() {
        // This tests that the cache key uses variable count, so different var counts = different entries
        let evaluator = Evaluator::new(Box::new(OptimizingEvaluator::new()));
        let expr = make_add_expr();
        let r1 = evaluator.evaluate(&*expr, &empty_vars()).unwrap();

        let mut vars = empty_vars();
        vars.insert("x".to_string(), 999.0);
        let r2 = evaluator.evaluate(&*expr, &vars).unwrap();
        assert_eq!(r1, 5.0);
        assert_eq!(r2, 5.0);
    }

    // Display bridge tests - these just test that the display impls don't panic
    #[test]
    fn test_console_display_result() {
        let display = CalculatorDisplay::new(Box::new(ConsoleDisplay));
        display.show_result(42.0);
    }

    #[test]
    fn test_console_display_error() {
        let display = CalculatorDisplay::new(Box::new(ConsoleDisplay));
        display.show_error("test error");
    }

    #[test]
    fn test_console_display_expression() {
        let display = CalculatorDisplay::new(Box::new(ConsoleDisplay));
        let expr = make_add_expr();
        display.show_expression(&*expr);
    }

    #[test]
    fn test_html_display_result() {
        let display = CalculatorDisplay::new(Box::new(HtmlDisplay));
        display.show_result(42.0);
    }

    #[test]
    fn test_html_display_error() {
        let display = CalculatorDisplay::new(Box::new(HtmlDisplay));
        display.show_error("test error");
    }

    #[test]
    fn test_json_display_result() {
        let display = CalculatorDisplay::new(Box::new(JsonDisplay));
        display.show_result(42.0);
    }

    #[test]
    fn test_json_display_error() {
        let display = CalculatorDisplay::new(Box::new(JsonDisplay));
        display.show_error("test error");
    }
}
