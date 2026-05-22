pub type ValidationResult = Result<(), String>;

pub struct Validators;

impl Validators {
    pub fn required(val: &str) -> ValidationResult {
        if val.trim().is_empty() {
            Err("This field is required".to_string())
        } else {
            Ok(())
        }
    }

    pub fn email(val: &str) -> ValidationResult {
        if !val.contains('@') || !val.contains('.') {
            Err("Please enter a valid email address".to_string())
        } else {
            Ok(())
        }
    }

    pub fn min_length(val: &str, min: usize) -> ValidationResult {
        if val.len() < min {
            Err(format!("Must be at least {} characters", min))
        } else {
            Ok(())
        }
    }

    pub fn range(val: f64, min: f64, max: f64) -> ValidationResult {
        if val < min || val > max {
            Err(format!("Value must be between {} and {}", min, max))
        } else {
            Ok(())
        }
    }
}
