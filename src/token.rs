#![allow(dead_code)]

// token.rs - Core token types and factory methods

// Number formats
#[derive(Debug, Clone, PartialEq)]
pub enum NumberFormat {
    Decimal,
    Scientific,
    Engineering,
}

// Basic token types
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Sqrt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    pub value: f64,
    pub format: NumberFormat,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            format: NumberFormat::Decimal,
        }
    }

    pub fn with_format(value: f64, format: NumberFormat) -> Self {
        Self { value, format }
    }

    pub fn format(&self) -> String {
        match self.format {
            NumberFormat::Decimal => format!("{}", self.value),
            NumberFormat::Scientific => format!("{:e}", self.value),
            NumberFormat::Engineering => {
                let exp = self.value.abs().log10().floor();
                let adj_exp = (exp - exp % 3.0).floor();
                let coeff = self.value / 10_f64.powf(adj_exp);
                format!("{}e{}", coeff, adj_exp)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    Operator(Operator),
    Function(Function),
    Variable(String),
    OpenParen,
    CloseParen,
}

// Factory methods for Token
impl Token {
    // Factory method for creating number tokens
    pub fn number(value: f64) -> Self {
        Self::Number(Number::new(value))
    }

    // Factory method for scientific notation
    pub fn scientific_number(value: f64) -> Self {
        Self::Number(Number::with_format(value, NumberFormat::Scientific))
    }

    // Factory method for operators
    pub fn operator(op: Operator) -> Self {
        Self::Operator(op)
    }

    // Factory method for functions
    pub fn function(func: Function) -> Self {
        Self::Function(func)
    }

    // Factory method for variables
    pub fn variable(name: impl Into<String>) -> Self {
        Self::Variable(name.into())
    }

    // Factory method from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        // Try parsing as a number first
        if let Ok(num) = s.parse::<f64>() {
            if s.contains('e') || s.contains('E') {
                return Ok(Self::Number(Number::with_format(
                    num,
                    NumberFormat::Scientific,
                )));
            }
            return Ok(Self::number(num));
        }

        // Check for operators
        match s {
            "+" => Ok(Self::operator(Operator::Add)),
            "-" => Ok(Self::operator(Operator::Subtract)),
            "*" => Ok(Self::operator(Operator::Multiply)),
            "/" => Ok(Self::operator(Operator::Divide)),
            "^" => Ok(Self::operator(Operator::Power)),
            // Functions
            "sin" => Ok(Self::function(Function::Sin)),
            "cos" => Ok(Self::function(Function::Cos)),
            "tan" => Ok(Self::function(Function::Tan)),
            "sqrt" => Ok(Self::function(Function::Sqrt)),
            // Parentheses
            "(" => Ok(Self::OpenParen),
            ")" => Ok(Self::CloseParen),
            // Must be a variable
            name if name.chars().all(|c| c.is_alphanumeric() || c == '_') => {
                Ok(Self::variable(name))
            }
            // Invalid token
            _ => Err(format!("Invalid token: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_format_decimal() {
        let num = Number::new(42.5);
        assert_eq!(num.value, 42.5);
        assert_eq!(num.format, NumberFormat::Decimal);
        assert_eq!(num.format(), "42.5");
    }

    #[test]
    fn test_number_format_scientific() {
        let num = Number::with_format(1234.0, NumberFormat::Scientific);
        assert_eq!(num.format(), "1.234e3");
    }

    #[test]
    fn test_number_format_engineering() {
        let num = Number::with_format(1500.0, NumberFormat::Engineering);
        assert_eq!(num.format(), "1.5e3");
    }

    #[test]
    fn test_token_from_str_number() {
        let tok = Token::from_str("42").unwrap();
        assert!(matches!(tok, Token::Number(n) if n.value == 42.0));
    }

    #[test]
    fn test_token_from_str_scientific() {
        let tok = Token::from_str("1e5").unwrap();
        assert!(
            matches!(tok, Token::Number(Number { value, format: NumberFormat::Scientific }) if value == 1e5)
        );
    }

    #[test]
    fn test_token_from_str_operators() {
        assert!(matches!(
            Token::from_str("+").unwrap(),
            Token::Operator(Operator::Add)
        ));
        assert!(matches!(
            Token::from_str("-").unwrap(),
            Token::Operator(Operator::Subtract)
        ));
        assert!(matches!(
            Token::from_str("*").unwrap(),
            Token::Operator(Operator::Multiply)
        ));
        assert!(matches!(
            Token::from_str("/").unwrap(),
            Token::Operator(Operator::Divide)
        ));
        assert!(matches!(
            Token::from_str("^").unwrap(),
            Token::Operator(Operator::Power)
        ));
    }

    #[test]
    fn test_token_from_str_functions() {
        assert!(matches!(
            Token::from_str("sin").unwrap(),
            Token::Function(Function::Sin)
        ));
        assert!(matches!(
            Token::from_str("cos").unwrap(),
            Token::Function(Function::Cos)
        ));
        assert!(matches!(
            Token::from_str("tan").unwrap(),
            Token::Function(Function::Tan)
        ));
        assert!(matches!(
            Token::from_str("sqrt").unwrap(),
            Token::Function(Function::Sqrt)
        ));
    }

    #[test]
    fn test_token_from_str_parens() {
        assert!(matches!(Token::from_str("(").unwrap(), Token::OpenParen));
        assert!(matches!(Token::from_str(")").unwrap(), Token::CloseParen));
    }

    #[test]
    fn test_token_from_str_variable() {
        let tok = Token::from_str("x").unwrap();
        assert!(matches!(tok, Token::Variable(name) if name == "x"));

        let tok = Token::from_str("_abc123").unwrap();
        assert!(matches!(tok, Token::Variable(name) if name == "_abc123"));
    }

    #[test]
    fn test_token_from_str_invalid() {
        assert!(Token::from_str("@").is_err());
        assert!(Token::from_str("!").is_err());
    }

    #[test]
    fn test_token_factories() {
        let t = Token::number(3.14);
        assert!(matches!(t, Token::Number(n) if n.value == 3.14));

        let t = Token::scientific_number(100.0);
        assert!(matches!(
            t,
            Token::Number(Number {
                format: NumberFormat::Scientific,
                ..
            })
        ));

        let t = Token::operator(Operator::Add);
        assert!(matches!(t, Token::Operator(Operator::Add)));

        let t = Token::function(Function::Sqrt);
        assert!(matches!(t, Token::Function(Function::Sqrt)));

        let t = Token::variable("my_var");
        assert!(matches!(t, Token::Variable(name) if name == "my_var"));
    }
}
