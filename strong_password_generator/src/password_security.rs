pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Custom(usize),
}

impl SecurityLevel {
    pub fn determine_password_criteria(input: &str) -> Result<Self, String> {
        match input {
            "Low" => Ok(Self::Low),
            "Medium" => Ok(Self::Medium),
            "High" => Ok(Self::High),
            custom_value => {
                if let Ok(value) = custom_value.parse::<usize>() {
                    if value >= 6 && value <= 99 {
                        Ok(Self::Custom(value))
                    } else {
                        Err("Custom value must be between 6 and 99.".to_string())
                    }
                } else {
                    Err("Invalid custom input. It must be a number.".to_string())
                }
            }
        }
    }

    pub fn security_value(&self) -> usize {
        match self {
            Self::Low => 6,
            Self::Medium => 12,
            Self::High => 16,
            Self::Custom(value) => *value,
        }
    }
}
