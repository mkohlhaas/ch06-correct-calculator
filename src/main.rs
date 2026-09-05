// #![allow(unused)]

// Correct Calculator - Chapter 6
// Demonstrates structural design patterns

// Import modules from Chapter 5
mod builder;
mod config;
mod factory;
mod token;

// Import new modules for Chapter 6
mod adapter;
mod bridge;
mod decorator;
mod expression;
mod facade;

use adapter::{ExternalLibraryAdapter, ScientificOperations, StandardScientificOperations};
use bridge::{
    CalculatorDisplay, ConsoleDisplay, Display, Evaluator, HtmlDisplay, JsonDisplay,
    OptimizingEvaluator, StandardEvaluator,
};
use config::{AngleMode, CalculatorConfig};
use decorator::{
    CachingExpression, ConsoleLogger, LoggingExpression, RangeValidatingExpression,
    TimingExpression,
};
use expression::{BinaryOperation, Expression, FunctionCall, NumberExpression, VariableExpression};
use facade::CalculatorFacade;
use std::collections::HashMap;
use std::f64::consts::PI;
use token::{Function, Operator};

fn main() {
    println!("Correct Calculator - Chapter 6 - Structural Patterns");

    // Build an expression tree for: 2 + 3 * 4
    let multiply_expr = Box::new(BinaryOperation {
        left: Box::new(NumberExpression { value: 3.0 }),
        right: Box::new(NumberExpression { value: 4.0 }),
        operator: Operator::Multiply,
    });

    let add_expr = Box::new(BinaryOperation {
        left: Box::new(NumberExpression { value: 2.0 }),
        right: multiply_expr,
        operator: Operator::Add,
    });

    let variables = HashMap::new();

    // ================= //
    // Composite Pattern //
    // ================= //

    // Demonstrate Composite Pattern
    println!("\n==================== Composite Pattern ====================\n");

    // Evaluate the expression
    println!("Expression: {}", add_expr);
    match add_expr.evaluate(&variables) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // More complex expression with a function call
    let sin_expr = Box::new(FunctionCall {
        function: Function::Sin,
        argument: Box::new(VariableExpression {
            name: "x".to_string(),
        }),
    });

    let mut var_map = HashMap::new();
    var_map.insert("x".to_string(), PI);

    println!("\nExpression: {}", sin_expr);
    match sin_expr.evaluate(&var_map) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // ================= //
    // Decorator Pattern //
    // ================= //

    // Demonstrate Decorator Pattern
    println!("\n==================== Decorator Pattern ====================\n");

    // ------- //
    // Logging //
    // ------- //

    // Create identical expressions for each decorator since we can't clone

    // Build an expression tree for: 2 + 3 * 4
    let add_for_logging = Box::new(BinaryOperation {
        left: Box::new(NumberExpression { value: 2.0 }),
        right: Box::new(BinaryOperation {
            left: Box::new(NumberExpression { value: 3.0 }),
            right: Box::new(NumberExpression { value: 4.0 }),
            operator: Operator::Multiply,
        }),
        operator: Operator::Add,
    });

    // Create a logging decorated expression
    let logging_expr = LoggingExpression::new(add_for_logging, Box::new(ConsoleLogger));

    println!("Evaluating with logging:");
    match logging_expr.evaluate(&variables) {
        Ok(result) => println!("Final result: {}", result),
        Err(e) => println!("Final error: {}", e),
    }

    // ------ //
    // Timing //
    // ------ //

    // Create another expression for timing
    let add_for_timing = Box::new(BinaryOperation {
        left: Box::new(NumberExpression { value: 2.0 }),
        right: Box::new(BinaryOperation {
            left: Box::new(NumberExpression { value: 3.0 }),
            right: Box::new(NumberExpression { value: 4.0 }),
            operator: Operator::Multiply,
        }),
        operator: Operator::Add,
    });

    // Create a timing decorated expression
    let timing_expr = TimingExpression::new(add_for_timing);

    println!("\nEvaluating with timing:");
    match timing_expr.evaluate(&variables) {
        Ok(result) => println!("Final result: {}", result),
        Err(e) => println!("Final error: {}", e),
    }

    // ------------------- //
    // Stacking Decorators //
    // ------------------- //

    // Every decorator implements/is an Expression -> we can stack decorators

    // Create another expression for logging and timing
    let add_for_logging_timing = BinaryOperation {
        left: Box::new(NumberExpression { value: 2.0 }),
        right: Box::new(BinaryOperation {
            left: Box::new(NumberExpression { value: 3.0 }),
            right: Box::new(NumberExpression { value: 4.0 }),
            operator: Operator::Multiply,
        }),
        operator: Operator::Add,
    };

    // Create a decorated expression with logging and timing
    let logging_timing_expr_inner = TimingExpression::new(Box::new(add_for_logging_timing));
    let logging_timing_expr_outer =
        LoggingExpression::new(Box::new(logging_timing_expr_inner), Box::new(ConsoleLogger));

    println!("\nEvaluating with logging and timing:");
    match logging_timing_expr_outer.evaluate(&variables) {
        Ok(result) => println!("Final result: {}", result),
        Err(e) => println!("Final error: {}", e),
    }

    // ------------------------ //
    // More Stacking Decorators //
    // ------------------------ //

    print!("\nAn example with more stacking:\n");

    let expr = Box::new(NumberExpression::new(42.0));
    let cached = Box::new(CachingExpression::new(expr));
    let validated = Box::new(RangeValidatingExpression::new(cached, 0.0, 100.0));
    let timed = Box::new(TimingExpression::new(validated));
    let logged = LoggingExpression::new(timed, Box::new(ConsoleLogger));
    let _result = logged.evaluate(&variables);

    // ------- //
    // Caching //
    // ------- //

    // Create another expression for caching
    let add_for_caching = Box::new(BinaryOperation {
        left: Box::new(NumberExpression { value: 2.0 }),
        right: Box::new(BinaryOperation {
            left: Box::new(NumberExpression { value: 3.0 }),
            right: Box::new(NumberExpression { value: 4.0 }),
            operator: Operator::Multiply,
        }),
        operator: Operator::Add,
    });

    // Create a caching decorated expression
    let caching_expr = CachingExpression::new(add_for_caching);

    println!("\nEvaluating with caching:");

    // calculate result
    match caching_expr.evaluate(&variables) {
        Ok(result) => println!("Final result: {}", result),
        Err(e) => println!("Final error: {}", e),
    }

    println!();

    // return cached result
    match caching_expr.evaluate(&variables) {
        Ok(result) => println!("Final result: {}", result),
        Err(e) => println!("Final error: {}", e),
    }

    // --------------- //
    // Range Decorator //
    // --------------- //

    // Create another expression for min,max ranges
    // Build an expression tree for: 2 + 3 * 4
    let add_for_ranges = Box::new(BinaryOperation {
        left: Box::new(NumberExpression { value: 2.0 }),
        right: Box::new(BinaryOperation {
            left: Box::new(NumberExpression { value: 3.0 }),
            right: Box::new(NumberExpression { value: 4.0 }),
            operator: Operator::Multiply,
        }),
        operator: Operator::Add,
    });

    // Create a decorated expression with ranges (change min, max to see difference)
    let range_expr = RangeValidatingExpression::new(add_for_ranges, 10.0, 20.0);

    println!("\nEvaluating with ranges:");
    match range_expr.evaluate(&variables) {
        Ok(result) => println!("Final result: {}", result),
        Err(e) => println!("Final error: {}", e),
    }

    // =============== //
    // Adapter Pattern //
    // =============== //

    // Demonstrate Adapter Pattern
    println!("\n==================== Adapter Pattern   ====================\n");

    // Create a standard scientific operations adapter
    let std_ops = StandardScientificOperations {
        angle_mode: AngleMode::Radians,
    };

    // Create an external library adapter
    let ext_ops = ExternalLibraryAdapter::new(AngleMode::Degrees);

    // Use both adapters
    println!("Standard sin(π/2): {}", std_ops.sin(PI / 2.0));
    println!("External sin(90°): {}", ext_ops.sin(90.0));

    // Demonstrate Facade Pattern
    println!("\n==================== Facade Pattern    ====================\n");

    // Create a calculator facade
    let mut calculator = CalculatorFacade::new(Box::new(std_ops), CalculatorConfig::default());

    // Use the simplified interface
    println!("Using calculator facade:");
    match calculator.evaluate("2 + 3 * 4") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Use convenience methods
    calculator.set_variable("a", 1.0);
    calculator.set_variable("b", -5.0);
    calculator.set_variable("c", 6.0);

    // Use specialized methods
    match calculator.calculate_quadratic(1.0, -5.0, 6.0) {
        Ok((x1, x2)) => println!("Quadratic roots: {} and {}", x1, x2),
        Err(e) => println!("Error: {}", e),
    }

    // Demonstrate Bridge Pattern
    println!("\n==================== Bridge Pattern    ====================\n");

    // Create different low-level implementations
    let console_impl = Box::new(ConsoleDisplay);
    let html_impl = Box::new(HtmlDisplay);
    let json_impl = Box::new(JsonDisplay);

    // Create displays with different implementations
    let console_display = CalculatorDisplay::new(console_impl);
    let html_display = CalculatorDisplay::new(html_impl);
    let json_display = CalculatorDisplay::new(json_impl);

    // Use the displays
    println!("Console display:");
    console_display.show_result(14.0);
    console_display.show_error("Sample error");
    console_display.show_expression(&*add_expr);

    println!("\nHTML display:");
    html_display.show_result(14.0);
    html_display.show_error("Sample error");
    html_display.show_expression(&*add_expr);

    println!("\nJSON display:");
    json_display.show_result(14.0);
    json_display.show_error("Sample error");
    json_display.show_expression(&*add_expr);

    // Demonstrate the evaluation bridge
    println!("\n==================== Evaluation Bridge ====================\n");

    // Create evaluation strategies
    let standard_eval = Box::new(StandardEvaluator);
    let optimizing_eval = Box::new(OptimizingEvaluator::new());

    // Create the evaluator with standard strategy
    let mut evaluator = Evaluator::new(standard_eval);

    // Use the evaluator
    println!("Standard evaluation:");
    match evaluator.evaluate(&*add_expr, &variables) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Change the strategy
    evaluator.change_strategy(optimizing_eval);

    // Use the evaluator with the new strategy
    println!("\nOptimizing evaluation:");
    match evaluator.evaluate(&*add_expr, &variables) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Evaluate again to demonstrate caching
    println!("\nOptimizing evaluation (second call, should be cached):");
    match evaluator.evaluate(&*add_expr, &variables) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nAll structural patterns have been demonstrated!");
}
