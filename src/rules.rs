use crate::errors::{CrackerError, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Rule engine for password mutations (similar to hashcat rules)
#[derive(Debug, Clone)]
pub enum Rule {
    None,                      // No mutation
    AppendDigit(u32),          // Append digit (0-999)
    PrependDigit(u32),         // Prepend digit (0-999)
    UppercaseFirst,            // Uppercase first character
    Lowercase,                 // Convert to lowercase
    Uppercase,                 // Convert to uppercase
    Reverse,                   // Reverse the string
    AppendSpecial(char),       // Append special character
    PrependSpecial(char),      // Prepend special character
    Duplicate,                 // Duplicate the password
    AppendYear(u32),           // Append year (2000-2030)
}

impl Rule {
    /// Apply rule to a password
    pub fn apply(&self, password: &str) -> String {
        match self {
            Rule::None => password.to_string(),
            Rule::AppendDigit(n) => format!("{}{}", password, n),
            Rule::PrependDigit(n) => format!("{}{}", n, password),
            Rule::UppercaseFirst => {
                let mut chars: Vec<char> = password.chars().collect();
                if let Some(first) = chars.first_mut() {
                    *first = first.to_uppercase().next().unwrap_or(*first);
                }
                chars.into_iter().collect()
            }
            Rule::Lowercase => password.to_lowercase(),
            Rule::Uppercase => password.to_uppercase(),
            Rule::Reverse => password.chars().rev().collect(),
            Rule::AppendSpecial(c) => format!("{}{}", password, c),
            Rule::PrependSpecial(c) => format!("{}{}", c, password),
            Rule::Duplicate => format!("{}{}", password, password),
            Rule::AppendYear(y) => format!("{}{}", password, y),
        }
    }

    /// Parse a rule from string format
    /// Format examples:
    /// - "append_digit:123"
    /// - "prepend_digit:5"
    /// - "uppercase_first"
    /// - "lowercase"
    /// - "reverse"
    /// - "append_special:!"
    pub fn parse(line: &str) -> Option<Rule> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        let parts: Vec<&str> = line.split(':').collect();

        match parts[0] {
            "none" => Some(Rule::None),
            "append_digit" => {
                if parts.len() == 2 {
                    parts[1].parse::<u32>().ok().map(Rule::AppendDigit)
                } else {
                    None
                }
            }
            "prepend_digit" => {
                if parts.len() == 2 {
                    parts[1].parse::<u32>().ok().map(Rule::PrependDigit)
                } else {
                    None
                }
            }
            "uppercase_first" => Some(Rule::UppercaseFirst),
            "lowercase" => Some(Rule::Lowercase),
            "uppercase" => Some(Rule::Uppercase),
            "reverse" => Some(Rule::Reverse),
            "append_special" => {
                if parts.len() == 2 && parts[1].len() == 1 {
                    parts[1].chars().next().map(Rule::AppendSpecial)
                } else {
                    None
                }
            }
            "prepend_special" => {
                if parts.len() == 2 && parts[1].len() == 1 {
                    parts[1].chars().next().map(Rule::PrependSpecial)
                } else {
                    None
                }
            }
            "duplicate" => Some(Rule::Duplicate),
            "append_year" => {
                if parts.len() == 2 {
                    parts[1].parse::<u32>().ok().map(Rule::AppendYear)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Rule engine that manages all rules
pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    /// Create default rule engine (no mutations)
    pub fn new() -> Self {
        Self {
            rules: vec![Rule::None],
        }
    }

    /// Load rules from file
    pub fn from_file(path: &str) -> Result<Self> {
        let file = File::open(path)
            .map_err(|e| CrackerError::RulesFileError(format!("Failed to open {}: {}", path, e)))?;

        let reader = BufReader::new(file);
        let mut rules = Vec::new();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| {
                CrackerError::RulesFileError(format!("Failed to read line {}: {}", line_num + 1, e))
            })?;

            if let Some(rule) = Rule::parse(&line) {
                rules.push(rule);
            }
        }

        if rules.is_empty() {
            rules.push(Rule::None);
        }

        Ok(Self { rules })
    }

    /// Create a rule engine with common default mutations
    pub fn default_rules() -> Self {
        let mut rules = vec![Rule::None];

        // Add common digit appends (0-999)
        for i in 0..1000 {
            rules.push(Rule::AppendDigit(i));
        }

        // Add common special character appends
        for c in ['!', '@', '#', '$', '%', '&', '*'] {
            rules.push(Rule::AppendSpecial(c));
        }

        // Add transformation rules
        rules.push(Rule::UppercaseFirst);
        rules.push(Rule::Lowercase);
        rules.push(Rule::Uppercase);
        rules.push(Rule::Reverse);

        // Add years (2000-2030)
        for year in 2000..=2030 {
            rules.push(Rule::AppendYear(year));
        }

        Self { rules }
    }

    /// Get all rules
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }

    /// Get number of rules
    pub fn count(&self) -> usize {
        self.rules.len()
    }

    /// Apply all rules to a password and return all candidates
    pub fn generate_candidates(&self, password: &str) -> Vec<String> {
        self.rules.iter().map(|rule| rule.apply(password)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_application() {
        assert_eq!(Rule::AppendDigit(123).apply("password"), "password123");
        assert_eq!(Rule::PrependDigit(99).apply("password"), "99password");
        assert_eq!(Rule::UppercaseFirst.apply("password"), "Password");
        assert_eq!(Rule::Lowercase.apply("PASSWORD"), "password");
        assert_eq!(Rule::Uppercase.apply("password"), "PASSWORD");
        assert_eq!(Rule::Reverse.apply("password"), "drowssap");
        assert_eq!(Rule::AppendSpecial('!').apply("password"), "password!");
    }

    #[test]
    fn test_rule_parsing() {
        assert!(matches!(Rule::parse("append_digit:123"), Some(Rule::AppendDigit(123))));
        assert!(matches!(Rule::parse("uppercase_first"), Some(Rule::UppercaseFirst)));
        assert!(matches!(Rule::parse("lowercase"), Some(Rule::Lowercase)));
        assert!(Rule::parse("invalid_rule").is_none());
    }
}
