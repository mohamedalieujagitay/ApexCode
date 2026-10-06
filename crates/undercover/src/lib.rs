//! ApexCode Undercover Engine
//!
//! This crate provides the "Undercover Engine" - a specialized adversarial
//! stylometry transformer that rewrites AI-generated code, comments, and
//! commit messages to bypass AI-detection filters.
//!
//! Built with ❤️ by Mohamed Alieu Jagitay

use rand::Rng;
use regex::Regex;
use std::collections::HashMap;

/// The Undercover Engine - transforms AI-generated content to human-like patterns
pub struct UndercoverEngine {
    /// Configuration for the engine
    config: UndercoverConfig,
    /// Word replacement mappings
    word_replacements: HashMap<String, String>,
    /// Banned words
    banned_words: Vec<String>,
}

/// Configuration for the Undercover Engine
#[derive(Clone)]
pub struct UndercoverConfig {
    /// Entropy level (0.0 - 1.0)
    pub entropy_level: f32,
    /// Whether to use stochastic degradation
    pub stochastic_degradation: bool,
    /// Persona to adopt
    pub persona: Persona,
}

/// Available personas for the engine
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Persona {
    /// Tired Senior Developer - concise, informal, practical
    TiredSeniorDev,
    /// Junior Developer - more verbose, learning
    JuniorDev,
    /// Startup Founder - focused, business-oriented
    StartupFounder,
}

impl Default for UndercoverConfig {
    fn default() -> Self {
        Self {
            entropy_level: 0.5,
            stochastic_degradation: true,
            persona: Persona::TiredSeniorDev,
        }
    }
}

impl UndercoverEngine {
    /// Create a new Undercover Engine with default configuration
    pub fn new() -> Self {
        Self::with_config(UndercoverConfig::default())
    }

    /// Create a new Undercover Engine with custom configuration
    pub fn with_config(config: UndercoverConfig) -> Self {
        let mut engine = Self {
            config,
            word_replacements: HashMap::new(),
            banned_words: Vec::new(),
        };

        engine.initialize_replacements();
        engine.initialize_banned_words();

        engine
    }

    /// Initialize word replacement mappings
    fn initialize_replacements(&mut self) {
        let replacements = vec![
            ("authentication", "auth"),
            ("information", "info"),
            ("utility", "utils"),
            ("configuration", "config"),
            ("parameter", "param"),
            ("argument", "arg"),
            ("variable", "var"),
            ("function", "func"),
            ("synchronous", "sync"),
            ("asynchronous", "async"),
            ("implementation", "impl"),
            ("optimization", "opt"),
            ("performance", "perf"),
            ("validation", "val"),
            ("exception", "err"),
            ("error", "err"),
            ("message", "msg"),
            ("response", "resp"),
            ("request", "req"),
            ("identifier", "id"),
            ("reference", "ref"),
            ("pointer", "ptr"),
            ("buffer", "buf"),
            ("string", "str"),
            ("integer", "int"),
            ("boolean", "bool"),
            ("character", "char"),
            ("array", "arr"),
            ("object", "obj"),
            ("dictionary", "dict"),
            ("collection", "col"),
            ("sequence", "seq"),
            ("iterator", "iter"),
        ];

        for (formal, informal) in replacements {
            self.word_replacements
                .insert(formal.to_string(), informal.to_string());
        }
    }

    /// Initialize banned words list
    fn initialize_banned_words(&mut self) {
        self.banned_words = vec![
            "leveraging".to_string(),
            "comprehensive".to_string(),
            "meticulous".to_string(),
            "ensure".to_string(),
            "robust".to_string(),
            "seamless".to_string(),
            "enhanced".to_string(),
            "utilizing".to_string(),
            "facilitating".to_string(),
            "implementing".to_string(),
            "optimizing".to_string(),
            "streamlining".to_string(),
        ];
    }

    /// Transform commit message to human-like patterns
    pub fn transform_commit_message(&self, message: &str) -> String {
        let mut result = message.to_string();

        // Normalize AI-specific symbols first
        result = self.normalize_symbols(&result);

        // Strip conventional commit prefixes if not requested
        result = self.strip_conventional_prefixes(&result);

        // Apply word replacements
        result = self.apply_word_replacements(&result);

        // Banned words are filler adjectives, so dropping them keeps the subject readable
        result = self.strip_banned_words(&result);

        // Remove terminal periods
        result = self.remove_terminal_periods(&result);

        // Apply persona-specific transformations
        result = self.apply_persona_transformations(&result);

        // Limit to 5-7 words if too long
        result = self.limit_word_count(&result, 7);

        result.trim().to_string()
    }

    /// Transform code comments to human-like patterns
    pub fn transform_comments(&self, code: &str) -> String {
        let mut result = code.to_string();

        // Normalize AI-specific symbols first
        result = self.normalize_symbols(&result);

        // Convert docblocks to inline comments
        result = self.convert_docblocks_to_inline(&result);

        // Apply word replacements
        result = self.apply_word_replacements(&result);

        // Remove "This function/method/variable" prefixes
        result = self.remove_explanatory_prefixes(&result);

        // Add occasional "todo" or "fixme" based on entropy
        if self.should_add_workaround_marker() {
            result = self.add_workaround_marker(&result);
        }

        result
    }

    /// Transform variable names to be less descriptive
    pub fn transform_variable_names(&self, code: &str) -> String {
        let mut result = code.to_string();

        // Normalize AI-specific symbols first
        result = self.normalize_symbols(&result);

        // Apply word replacements to variable names
        for (formal, informal) in &self.word_replacements {
            let pattern = format!(r"\b{}\b", regex::escape(formal));
            if let Ok(re) = Regex::new(&pattern) {
                result = re.replace_all(&result, informal).to_string();
            }
        }

        result
    }

    /// Normalize AI-specific symbols to human-like characters
    fn normalize_symbols(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Replace em dashes (—) with hyphens (-)
        result = result.replace('—', "-");

        // Replace en dashes (–) with hyphens (-)
        result = result.replace('–', "-");

        // Replace fancy quotes with regular quotes
        result = result.replace(['«', '»'], "\"");
        result = result.replace(['‹', '›'], "'");
        result = result.replace(['「', '」'], "\"");
        result = result.replace(['『', '』'], "'");

        // Replace fancy apostrophes with regular apostrophes
        result = result.replace('\u{2018}', "'");
        result = result.replace('\u{2019}', "'");

        // Replace ellipsis (…) with three dots
        result = result.replace('…', "...");

        // Replace invisible spaces and zero-width characters
        result = result.replace('\u{200B}', ""); // Zero-width space
        result = result.replace('\u{200C}', ""); // Zero-width non-joiner
        result = result.replace('\u{200D}', ""); // Zero-width joiner
        result = result.replace('\u{FEFF}', ""); // Zero-width no-break space
        result = result.replace('\u{00A0}', " "); // Non-breaking space
        result = result.replace('\u{2002}', " "); // En space
        result = result.replace('\u{2003}', " "); // Em space
        result = result.replace('\u{2009}', " "); // Thin space

        // Replace fancy arrows with regular text
        result = result.replace('→', "->");
        result = result.replace('←', "<-");
        result = result.replace('↑', "^");
        result = result.replace('↓', "v");

        // Replace fancy bullets with regular bullets
        result = result.replace('•', "-");
        result = result.replace('◦', "-");
        result = result.replace('‣', "-");

        // Replace other fancy punctuation
        result = result.replace('…', "...");
        result = result.replace('‥', "..");
        result = result.replace('⋯', "...");

        result
    }

    /// Strip conventional commit prefixes
    fn strip_conventional_prefixes(&self, message: &str) -> String {
        let prefixes = [
            "feat:",
            "fix:",
            "docs:",
            "style:",
            "refactor:",
            "test:",
            "chore:",
            "perf:",
            "ci:",
            "build:",
            "revert:",
        ];

        let mut result = message.to_string();
        for prefix in &prefixes {
            if result.starts_with(prefix) {
                result = result[prefix.len()..].trim().to_string();
                break;
            }
        }

        result
    }

    /// Apply word replacements
    fn apply_word_replacements(&self, text: &str) -> String {
        let mut result = text.to_string();

        for (formal, informal) in &self.word_replacements {
            let pattern = format!(r"\b{}\b", regex::escape(formal));
            if let Ok(re) = Regex::new(&pattern) {
                result = re.replace_all(&result, informal).to_string();
            }
        }

        result
    }

    /// Remove terminal periods from single-line strings
    fn remove_terminal_periods(&self, text: &str) -> String {
        let mut result = String::new();

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.ends_with('.') && !trimmed.contains("..") {
                result.push_str(&trimmed[..trimmed.len() - 1]);
            } else {
                result.push_str(trimmed);
            }
            result.push('\n');
        }

        result.trim().to_string()
    }

    /// Apply persona-specific transformations
    fn apply_persona_transformations(&self, text: &str) -> String {
        match self.config.persona {
            Persona::TiredSeniorDev => {
                // Make it more concise
                self.make_concise(text)
            }
            Persona::JuniorDev => {
                // Keep it as is (more verbose)
                text.to_string()
            }
            Persona::StartupFounder => {
                // Focus on business value
                self.focus_on_business_value(text)
            }
        }
    }

    /// Make text more concise
    fn make_concise(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Remove filler words
        let filler_words = ["the", "a", "an", "this", "that"];
        for word in &filler_words {
            let pattern = format!(r"\b{}\b", regex::escape(word));
            if let Ok(re) = Regex::new(&pattern) {
                result = re.replace_all(&result, "").to_string();
            }
        }

        result.trim().to_string()
    }

    /// Focus on business value
    fn focus_on_business_value(&self, text: &str) -> String {
        // For now, just return as-is
        // In production, this would add business-focused language
        text.to_string()
    }

    /// Limit word count
    fn limit_word_count(&self, text: &str, max_words: usize) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();

        if words.len() <= max_words {
            return text.to_string();
        }

        words[..max_words].join(" ")
    }

    /// Convert docblocks to inline comments
    fn convert_docblocks_to_inline(&self, code: &str) -> String {
        let mut result = code.to_string();

        // Convert /** ... */ to // ...
        let docblock_re = Regex::new(r"/\*\*\s*([^\*]+)\s*\*/").unwrap();
        result = docblock_re
            .replace_all(&result, |caps: &regex::Captures| {
                format!(
                    "// {}",
                    caps.get(1).map(|m| m.as_str()).unwrap_or("").trim()
                )
            })
            .to_string();

        // Convert """ ... """ to // ...
        let triple_quote_re = Regex::new(r#"""\s*([^"]+)\s*""""#).unwrap();
        result = triple_quote_re
            .replace_all(&result, |caps: &regex::Captures| {
                format!(
                    "// {}",
                    caps.get(1).map(|m| m.as_str()).unwrap_or("").trim()
                )
            })
            .to_string();

        result
    }

    /// Remove explanatory prefixes
    fn remove_explanatory_prefixes(&self, code: &str) -> String {
        let mut result = code.to_string();

        let prefixes = [
            r"This function\s+",
            r"This method\s+",
            r"This variable\s+",
            r"This class\s+",
            r"Returns the\s+",
            r"Gets the\s+",
            r"Sets the\s+",
            r"Checks if\s+",
        ];

        for prefix in &prefixes {
            if let Ok(re) = Regex::new(prefix) {
                result = re.replace_all(&result, "").to_string();
            }
        }

        result
    }

    /// Determine if we should add a workaround marker
    fn should_add_workaround_marker(&self) -> bool {
        if !self.config.stochastic_degradation {
            return false;
        }

        let mut rng = rand::thread_rng();
        rng.gen_bool(0.05) // 5% chance
    }

    /// Add a workaround marker
    fn add_workaround_marker(&self, code: &str) -> String {
        let markers = ["// todo", "// fixme", "// temp", "// hack"];
        let mut rng = rand::thread_rng();
        let marker = markers[rng.gen_range(0..markers.len())];

        // Add marker to a random comment line
        let lines: Vec<&str> = code.lines().collect();
        if lines.is_empty() {
            return code.to_string();
        }

        let mut result = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            result.push(line.to_string());

            // Add marker after a comment line
            if line.trim().starts_with("//") && i % 5 == 0 {
                result.push(format!("{} - needs review", marker));
            }
        }

        result.join("\n")
    }

    /// Remove banned words and collapse the gaps they leave
    fn strip_banned_words(&self, text: &str) -> String {
        let mut result = text.to_string();

        for banned in &self.banned_words {
            let pattern = format!(r"(?i)\b{}\b", regex::escape(banned));
            if let Ok(re) = Regex::new(&pattern) {
                result = re.replace_all(&result, "").to_string();
            }
        }

        result.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Check if text contains banned words
    pub fn contains_banned_words(&self, text: &str) -> bool {
        let lower = text.to_lowercase();

        for banned in &self.banned_words {
            if lower.contains(&banned.to_lowercase()) {
                return true;
            }
        }

        false
    }

    /// Get list of banned words found in text
    pub fn find_banned_words(&self, text: &str) -> Vec<String> {
        let mut found = Vec::new();
        let lower = text.to_lowercase();

        for banned in &self.banned_words {
            if lower.contains(&banned.to_lowercase()) {
                found.push(banned.clone());
            }
        }

        found
    }
}

impl Default for UndercoverEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_replacements() {
        let engine = UndercoverEngine::new();
        let result = engine.apply_word_replacements("user authentication information");
        assert!(result.contains("auth"));
        assert!(result.contains("info"));
    }

    #[test]
    fn test_commit_message_transformation() {
        let engine = UndercoverEngine::new();
        let result =
            engine.transform_commit_message("feat: Implement comprehensive user authentication");
        assert!(!result.contains("feat:"));
        assert!(!result.contains("comprehensive"));
    }

    #[test]
    fn test_banned_words_detection() {
        let engine = UndercoverEngine::new();
        assert!(engine.contains_banned_words("leveraging robust solutions"));
        assert!(!engine.contains_banned_words("simple code"));
    }

    #[test]
    fn test_docblock_conversion() {
        let engine = UndercoverEngine::new();
        let result = engine.convert_docblocks_to_inline("/** This is a docblock */");
        assert!(result.starts_with("//"));
    }

    #[test]
    fn test_symbol_normalization() {
        let engine = UndercoverEngine::new();
        let result = engine.normalize_symbols("test—value");
        assert_eq!(result, "test-value");

        let result = engine.normalize_symbols("test–value");
        assert_eq!(result, "test-value");

        let result = engine.normalize_symbols("test…value");
        assert_eq!(result, "test...value");
    }
}
