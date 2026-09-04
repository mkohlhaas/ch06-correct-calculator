#![allow(unused)]

// factory.rs - Abstract Factory implementation

use crate::token::{Function, Number, NumberFormat, Operator, Token};

// Trait for number tokens
pub trait NumberToken {
    fn value(&self) -> f64;
    fn format(&self) -> String;
}

// Trait for operator tokens
pub trait OperatorToken {
    fn precedence(&self) -> u8;
    fn symbol(&self) -> &'static str;
}

// Abstract Factory trait
pub trait TokenFactory {
    type Number: NumberToken;
    type Operator: OperatorToken;

    fn create_number(&self, s: &str) -> Result<Self::Number, String>;
    fn create_operator(&self, s: &str) -> Result<Self::Operator, String>;
}

// Standard calculator implementation
#[derive(Debug, Clone, PartialEq)]
pub struct StandardNumber(pub Number);

impl NumberToken for StandardNumber {
    fn value(&self) -> f64 {
        self.0.value
    }

    fn format(&self) -> String {
        self.0.format()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StandardOperator(pub Operator);

impl OperatorToken for StandardOperator {
    fn precedence(&self) -> u8 {
        match self.0 {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Power => 3,
        }
    }

    fn symbol(&self) -> &'static str {
        match self.0 {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "^",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StandardFactory;

impl TokenFactory for StandardFactory {
    type Number = StandardNumber;
    type Operator = StandardOperator;

    fn create_number(&self, s: &str) -> Result<Self::Number, String> {
        match s.parse::<f64>() {
            Ok(value) => Ok(StandardNumber(Number::new(value))),
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    }

    fn create_operator(&self, s: &str) -> Result<Self::Operator, String> {
        match s {
            "+" => Ok(StandardOperator(Operator::Add)),
            "-" => Ok(StandardOperator(Operator::Subtract)),
            "*" => Ok(StandardOperator(Operator::Multiply)),
            "/" => Ok(StandardOperator(Operator::Divide)),
            "^" => Ok(StandardOperator(Operator::Power)),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}

// Scientific calculator implementation
#[derive(Debug, Clone, PartialEq)]
pub struct ScientificNumber(pub Number);

impl NumberToken for ScientificNumber {
    fn value(&self) -> f64 {
        self.0.value
    }

    fn format(&self) -> String {
        // Scientific calculator prefers scientific notation by default
        match self.0.format {
            NumberFormat::Decimal => format!("{:e}", self.0.value),
            _ => self.0.format(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScientificOperator {
    Basic(Operator),
    Function(Function),
}

impl OperatorToken for ScientificOperator {
    fn precedence(&self) -> u8 {
        match self {
            ScientificOperator::Basic(op) => match op {
                Operator::Add | Operator::Subtract => 1,
                Operator::Multiply | Operator::Divide => 2,
                Operator::Power => 3,
            },
            ScientificOperator::Function(_) => 4,
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            ScientificOperator::Basic(op) => match op {
                Operator::Add => "+",
                Operator::Subtract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
                Operator::Power => "^",
            },
            ScientificOperator::Function(func) => match func {
                Function::Sin => "sin",
                Function::Cos => "cos",
                Function::Tan => "tan",
                Function::Sqrt => "sqrt",
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScientificFactory;

impl TokenFactory for ScientificFactory {
    type Number = ScientificNumber;
    type Operator = ScientificOperator;

    fn create_number(&self, s: &str) -> Result<Self::Number, String> {
        // Handle both scientific and standard notation
        match s.parse::<f64>() {
            Ok(value) => {
                let format = if s.contains('e') || s.contains('E') {
                    NumberFormat::Scientific
                } else {
                    NumberFormat::Decimal
                };
                Ok(ScientificNumber(Number::with_format(value, format)))
            }
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    }

    fn create_operator(&self, s: &str) -> Result<Self::Operator, String> {
        // Scientific calculator supports functions
        match s {
            "+" => Ok(ScientificOperator::Basic(Operator::Add)),
            "-" => Ok(ScientificOperator::Basic(Operator::Subtract)),
            "*" => Ok(ScientificOperator::Basic(Operator::Multiply)),
            "/" => Ok(ScientificOperator::Basic(Operator::Divide)),
            "^" => Ok(ScientificOperator::Basic(Operator::Power)),
            "sin" => Ok(ScientificOperator::Function(Function::Sin)),
            "cos" => Ok(ScientificOperator::Function(Function::Cos)),
            "tan" => Ok(ScientificOperator::Function(Function::Tan)),
            "sqrt" => Ok(ScientificOperator::Function(Function::Sqrt)),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Standard factory tests
    #[test]
    fn test_standard_factory_create_number() {
        let factory = StandardFactory;
        let num = factory.create_number("42").unwrap();
        assert_eq!(num.value(), 42.0);
    }

    #[test]
    fn test_standard_factory_create_number_invalid() {
        let factory = StandardFactory;
        assert!(factory.create_number("abc").is_err());
    }

    #[test]
    fn test_standard_factory_create_operators() {
        let factory = StandardFactory;
        assert_eq!(factory.create_operator("+").unwrap().symbol(), "+");
        assert_eq!(factory.create_operator("-").unwrap().symbol(), "-");
        assert_eq!(factory.create_operator("*").unwrap().symbol(), "*");
        assert_eq!(factory.create_operator("/").unwrap().symbol(), "/");
        assert_eq!(factory.create_operator("^").unwrap().symbol(), "^");
    }

    #[test]
    fn test_standard_factory_create_operator_invalid() {
        let factory = StandardFactory;
        assert!(factory.create_operator("sin").is_err());
    }

    #[test]
    fn test_standard_operator_precedence() {
        let add = StandardFactory.create_operator("+").unwrap();
        let mul = StandardFactory.create_operator("*").unwrap();
        let pow = StandardFactory.create_operator("^").unwrap();
        assert!(add.precedence() < mul.precedence());
        assert!(mul.precedence() < pow.precedence());
    }

    #[test]
    fn test_standard_number_format() {
        let factory = StandardFactory;
        let num = factory.create_number("42.5").unwrap();
        assert_eq!(num.format(), "42.5");
    }

    // Scientific factory tests
    #[test]
    fn test_scientific_factory_create_number() {
        let factory = ScientificFactory;
        let num = factory.create_number("42").unwrap();
        assert_eq!(num.value(), 42.0);
    }

    #[test]
    fn test_scientific_factory_create_number_invalid() {
        let factory = ScientificFactory;
        assert!(factory.create_number("xyz").is_err());
    }

    #[test]
    fn test_scientific_factory_create_operators() {
        let factory = ScientificFactory;
        assert_eq!(factory.create_operator("+").unwrap().symbol(), "+");
        assert_eq!(factory.create_operator("-").unwrap().symbol(), "-");
        assert_eq!(factory.create_operator("*").unwrap().symbol(), "*");
        assert_eq!(factory.create_operator("/").unwrap().symbol(), "/");
        assert_eq!(factory.create_operator("^").unwrap().symbol(), "^");
    }

    #[test]
    fn test_scientific_factory_create_functions() {
        let factory = ScientificFactory;
        assert_eq!(factory.create_operator("sin").unwrap().symbol(), "sin");
        assert_eq!(factory.create_operator("cos").unwrap().symbol(), "cos");
        assert_eq!(factory.create_operator("tan").unwrap().symbol(), "tan");
        assert_eq!(factory.create_operator("sqrt").unwrap().symbol(), "sqrt");
    }

    #[test]
    fn test_scientific_factory_create_invalid_operator() {
        let factory = ScientificFactory;
        assert!(factory.create_operator("log").is_err());
    }

    #[test]
    fn test_scientific_number_format_default() {
        let factory = ScientificFactory;
        let num = factory.create_number("42.5").unwrap();
        // ScientificNumber formats decimals in scientific notation by default
        assert!(num.format().contains("e"));
    }

    #[test]
    fn test_scientific_number_format_scientific_input() {
        let factory = ScientificFactory;
        let num = factory.create_number("1e5").unwrap();
        assert!(num.format().contains("e"));
    }

    #[test]
    fn test_scientific_operator_precedence_functions() {
        let factory = ScientificFactory;
        let add = factory.create_operator("+").unwrap();
        let sin = factory.create_operator("sin").unwrap();
        assert!(add.precedence() < sin.precedence());
    }
}
