#![allow(unused)]

// config.rs - Configuration (alternative to Singleton)

use crate::token::NumberFormat;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub enum AngleMode {
    Degrees,
    Radians,
}

#[derive(Debug, Clone)]
pub struct CalculatorConfig {
    pub precision: u32,
    pub angle_mode: AngleMode,
    pub notation: NumberFormat,
}

impl Default for CalculatorConfig {
    fn default() -> Self {
        Self {
            precision: 10,
            angle_mode: AngleMode::Radians,
            notation: NumberFormat::Decimal,
        }
    }
}

impl CalculatorConfig {
    // Factory methods for common configurations
    pub fn scientific() -> Self {
        Self {
            precision: 15,
            angle_mode: AngleMode::Radians,
            notation: NumberFormat::Scientific,
        }
    }

    pub fn engineering() -> Self {
        Self {
            notation: NumberFormat::Engineering,
            ..Default::default()
        }
    }
}

// Constants
pub const DEFAULT_PRECISION: u32 = 10;
pub const MAX_PRECISION: u32 = 100;

// If we need a global configuration (alternative to Singleton)
static CONFIG: OnceLock<CalculatorConfig> = OnceLock::new();

pub fn get_global_config() -> &'static CalculatorConfig {
    CONFIG.get_or_init(|| {
        // In a real application, this might load from a file or environment
        CalculatorConfig::default()
    })
}

// Flyweight Pattern

// Thread-safe calculator with shared config
use std::sync::{Arc, Mutex};

pub struct CalculatorPool {
    shared_config: Arc<CalculatorConfig>,
    // In a real application, this would store calculator instances
    _calculators: Vec<()>,
}

impl CalculatorPool {
    pub fn new(config: CalculatorConfig) -> Self {
        Self {
            shared_config: Arc::new(config),
            _calculators: Vec::new(),
        }
    }

    pub fn get_config(&self) -> Arc<CalculatorConfig> {
        Arc::clone(&self.shared_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CalculatorConfig::default();
        assert_eq!(config.precision, 10);
        assert!(matches!(config.angle_mode, AngleMode::Radians));
        assert!(matches!(config.notation, NumberFormat::Decimal));
    }

    #[test]
    fn test_scientific_config() {
        let config = CalculatorConfig::scientific();
        assert_eq!(config.precision, 15);
        assert!(matches!(config.angle_mode, AngleMode::Radians));
        assert!(matches!(config.notation, NumberFormat::Scientific));
    }

    #[test]
    fn test_engineering_config() {
        let config = CalculatorConfig::engineering();
        assert!(matches!(config.notation, NumberFormat::Engineering));
        // Should inherit defaults for other fields
        assert_eq!(config.precision, 10);
        assert!(matches!(config.angle_mode, AngleMode::Radians));
    }

    #[test]
    fn test_config_clone() {
        let config = CalculatorConfig::default();
        let cloned = config.clone();
        assert_eq!(cloned.precision, config.precision);
    }

    #[test]
    fn test_global_config_returns_same_instance() {
        let a = get_global_config();
        let b = get_global_config();
        // Both should point to the same static instance
        let a_ptr = a as *const CalculatorConfig;
        let b_ptr = b as *const CalculatorConfig;
        assert_eq!(a_ptr, b_ptr);
    }

    #[test]
    fn test_global_config_is_default() {
        let config = get_global_config();
        assert_eq!(config.precision, 10);
        assert!(matches!(config.angle_mode, AngleMode::Radians));
    }

    #[test]
    fn test_calculator_pool_shares_config() {
        let config = CalculatorConfig::default();
        let pool = CalculatorPool::new(config);
        let shared = pool.get_config();
        assert_eq!(shared.precision, 10);
    }

    #[test]
    fn test_calculator_pool_multiple_gets() {
        let pool = CalculatorPool::new(CalculatorConfig::scientific());
        let a = pool.get_config();
        let b = pool.get_config();
        assert_eq!(a.precision, b.precision);
    }

    #[test]
    fn test_angle_mode_debug() {
        assert_eq!(format!("{:?}", AngleMode::Degrees), "Degrees");
        assert_eq!(format!("{:?}", AngleMode::Radians), "Radians");
    }
}
