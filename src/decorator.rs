#![allow(unused)]

// decorator.rs - Decorator pattern implementation

use crate::expression::Expression;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::time::Instant;

// =================== //
// 1. Logger Decorator //
// =================== //

// Logger trait for logging operations
pub trait Logger {
    fn log(&self, message: &str);
}

// Console logger implementation
pub struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[LOG] {}", message);
    }
}

// A decorator for expressions that logs evaluation
// NOTE: the use of generics would be better. No need to change inner and logger during run-time.
pub struct LoggingExpression {
    inner: Box<dyn Expression>,
    logger: Box<dyn Logger>,
}

impl LoggingExpression {
    pub fn new(inner: Box<dyn Expression>, logger: Box<dyn Logger>) -> Self {
        Self { inner, logger }
    }
}

impl Expression for LoggingExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        self.logger.log(&format!("Evaluating: {}", self.inner));
        let result = self.inner.evaluate(variables);
        match &result {
            Ok(value) => self.logger.log(&format!("Result: {}", value)),
            Err(err) => self.logger.log(&format!("Error: {}", err)),
        }
        result
    }

    fn precedence(&self) -> u8 {
        self.inner.precedence()
    }
}

impl Display for LoggingExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

// ================== //
// 2. Timer Decorator //
// ================== //

// A decorator that times evaluation
pub struct TimingExpression {
    inner: Box<dyn Expression>,
}

impl TimingExpression {
    pub fn new(inner: Box<dyn Expression>) -> Self {
        Self { inner }
    }
}

impl Expression for TimingExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let start = Instant::now();
        let result = self.inner.evaluate(variables);
        let duration = start.elapsed();
        println!("Evaluation took: {:?}", duration);
        result
    }

    fn precedence(&self) -> u8 {
        self.inner.precedence()
    }
}

impl Display for TimingExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

// ==================== //
// 3. Caching Decorator //
// ==================== //

// A decorator that caches evaluation results
use std::cell::RefCell;

// The CachingExpression introduces an interesting Rust challenge. Caching requires storing a
// result, which involves mutating internal state (storing the computed result so future calls can
// return it without re-evaluating), but evaluate takes &self , not &mut self . This is where
// interior mutability comes in using RefCell.

pub struct CachingExpression {
    inner: Box<dyn Expression>,
    // Using RefCell for interior mutability
    last_result: RefCell<Option<f64>>, // we need interior mutability bc `evaluate` takes an immutable
                                       // reference to self (&self; not &mut self)
}

impl CachingExpression {
    pub fn new(inner: Box<dyn Expression>) -> Self {
        Self {
            inner,
            last_result: RefCell::new(None),
        }
    }

    pub fn invalidate_cache(&self) {
        *self.last_result.borrow_mut() = None;
    }
}

impl Expression for CachingExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        // In a real implementation, we would need to check if variables have changed
        // For this example, we're keeping it simple
        if let Some(result) = *self.last_result.borrow() {
            println!("Returning cached result");
            return Ok(result);
        } // immutable borrow is dropped here (borrow scope is obvious and short-lived)

        let result = self.inner.evaluate(variables)?;
        // Using interior mutability with RefCell
        *self.last_result.borrow_mut() = Some(result);

        println!("Calculating result");
        Ok(result)
    }

    fn precedence(&self) -> u8 {
        self.inner.precedence()
    }
}

impl Display for CachingExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

// ============================ //
// 4. Range Validator Decorator //
// ============================ //

// A decorator that validates the result range
pub struct RangeValidatingExpression {
    inner: Box<dyn Expression>,
    min: f64,
    max: f64,
}

impl RangeValidatingExpression {
    pub fn new(inner: Box<dyn Expression>, min: f64, max: f64) -> Self {
        // TODO: it should be checked if max >= min
        Self { inner, min, max }
    }
}

impl Expression for RangeValidatingExpression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        let result = self.inner.evaluate(variables)?;

        if result < self.min {
            Err(format!(
                "Result {} is less than minimum {}",
                result, self.min
            ))
        } else if result > self.max {
            Err(format!(
                "Result {} is greater than maximum {}",
                result, self.max
            ))
        } else {
            Ok(result)
        }
    }

    fn precedence(&self) -> u8 {
        self.inner.precedence()
    }
}

impl Display for RangeValidatingExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "validate({}, min={}, max={})",
            self.inner, self.min, self.max
        )
    }
}

// ===== //
// Tests //
// ===== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::NumberExpression;

    fn empty_vars() -> HashMap<String, f64> {
        HashMap::new()
    }

    fn make_number(val: f64) -> Box<dyn Expression> {
        Box::new(NumberExpression::new(val))
    }

    #[test]
    fn test_logging_expression_evaluate() {
        let inner = make_number(42.0);
        let expr = LoggingExpression::new(inner, Box::new(ConsoleLogger));
        let result = expr.evaluate(&empty_vars()).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_logging_expression_display() {
        let inner = make_number(42.0);
        let expr = LoggingExpression::new(inner, Box::new(ConsoleLogger));
        assert_eq!(format!("{}", expr), "42");
    }

    #[test]
    fn test_timing_expression_evaluate() {
        let inner = make_number(7.0);
        let expr = TimingExpression::new(inner);
        let result = expr.evaluate(&empty_vars()).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_timing_expression_display() {
        let inner = make_number(7.0);
        let expr = TimingExpression::new(inner);
        assert_eq!(format!("{}", expr), "7");
    }

    #[test]
    fn test_caching_expression_first_call() {
        let inner = make_number(99.0);
        let expr = CachingExpression::new(inner);
        let result = expr.evaluate(&empty_vars()).unwrap();
        assert_eq!(result, 99.0);
    }

    #[test]
    fn test_caching_expression_second_call_uses_cache() {
        let inner = make_number(99.0);
        let expr = CachingExpression::new(inner);
        let _ = expr.evaluate(&empty_vars()).unwrap();
        // Second call should hit cache
        let result = expr.evaluate(&empty_vars()).unwrap();
        assert_eq!(result, 99.0);
    }

    #[test]
    fn test_caching_expression_display() {
        let inner = make_number(99.0);
        let expr = CachingExpression::new(inner);
        assert_eq!(format!("{}", expr), "99");
    }

    #[test]
    fn test_caching_expression_invalidates() {
        let inner = make_number(5.0);
        let expr = CachingExpression::new(inner);
        let _ = expr.evaluate(&empty_vars()).unwrap();
        expr.invalidate_cache();
        // Should re-evaluate (but same result)
        let result = expr.evaluate(&empty_vars()).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_range_validating_in_range() {
        let inner = make_number(15.0);
        let expr = RangeValidatingExpression::new(inner, 10.0, 20.0);
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 15.0);
    }

    #[test]
    fn test_range_validating_below_min() {
        let inner = make_number(5.0);
        let expr = RangeValidatingExpression::new(inner, 10.0, 20.0);
        assert!(expr.evaluate(&empty_vars()).is_err());
    }

    #[test]
    fn test_range_validating_above_max() {
        let inner = make_number(25.0);
        let expr = RangeValidatingExpression::new(inner, 10.0, 20.0);
        assert!(expr.evaluate(&empty_vars()).is_err());
    }

    #[test]
    fn test_range_validating_boundary() {
        let inner = make_number(10.0);
        let expr = RangeValidatingExpression::new(inner, 10.0, 20.0);
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 10.0);

        let inner = make_number(20.0);
        let expr = RangeValidatingExpression::new(inner, 10.0, 20.0);
        assert_eq!(expr.evaluate(&empty_vars()).unwrap(), 20.0);
    }

    #[test]
    fn test_range_validating_display() {
        let inner = make_number(15.0);
        let expr = RangeValidatingExpression::new(inner, 10.0, 20.0);
        assert_eq!(format!("{}", expr), "validate(15, min=10, max=20)");
    }

    #[test]
    fn test_stacked_decorators() {
        // Caching -> RangeValidating -> Logging
        let inner = make_number(42.0);
        let cached = Box::new(CachingExpression::new(inner));
        let validated = Box::new(RangeValidatingExpression::new(cached, 0.0, 100.0));
        let logged = LoggingExpression::new(validated, Box::new(ConsoleLogger));
        let result = logged.evaluate(&empty_vars()).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_stacked_decorators_out_of_range() {
        let inner = make_number(200.0);
        let validated = Box::new(RangeValidatingExpression::new(inner, 0.0, 100.0));
        let logged = LoggingExpression::new(validated, Box::new(ConsoleLogger));
        assert!(logged.evaluate(&empty_vars()).is_err());
    }
}
