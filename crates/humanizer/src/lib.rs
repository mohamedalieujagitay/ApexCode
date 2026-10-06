//! ApexCode Humanizer Engine
//!
//! This crate provides code transformation capabilities to match
//! repository-specific style and inject human entropy.

use apexcode_core::{
    CodeBlock, CommentStyle, CommitFormat, Humanizer, IndentationStyle, NamingConvention,
    StyleProfile,
};
use git2::Repository;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Main humanizer implementation
pub struct ApexHumanizer {
    config: HumanizerConfig,
    style_cache: HashMap<PathBuf, StyleProfile>,
}

/// Humanizer configuration
#[derive(Clone)]
pub struct HumanizerConfig {
    /// Entropy level (0.0 - 1.0)
    pub entropy_level: f32,
    /// Whether to preserve comments
    pub preserve_comments: bool,
    /// Whether to inject new comments
    pub inject_comments: bool,
    /// Whether to rename variables
    pub rename_variables: bool,
}

impl Default for HumanizerConfig {
    fn default() -> Self {
        Self {
            entropy_level: 0.5,
            preserve_comments: true,
            inject_comments: false,
            rename_variables: false,
        }
    }
}

impl Default for ApexHumanizer {
    fn default() -> Self {
        Self::new()
    }
}

impl ApexHumanizer {
    /// Create a new humanizer with default configuration
    pub fn new() -> Self {
        Self::with_config(HumanizerConfig::default())
    }

    /// Create a new humanizer with custom configuration
    pub fn with_config(config: HumanizerConfig) -> Self {
        Self {
            config,
            style_cache: HashMap::new(),
        }
    }

    /// Apply entropy injection to code
    fn apply_entropy(&self, code: &str, profile: &StyleProfile) -> String {
        let mut result = code.to_string();
        let entropy = self.config.entropy_level;

        // Apply line length variation
        if entropy > 0.3 {
            result = self.vary_line_length(&result, profile);
        }

        // Apply comment injection if enabled
        if self.config.inject_comments && entropy > 0.5 {
            result = self.inject_comments(&result, profile);
        }

        // Apply whitespace variation
        if entropy > 0.2 {
            result = self.vary_whitespace(&result, profile);
        }

        result
    }

    /// Vary line lengths to appear more human
    fn vary_line_length(&self, code: &str, profile: &StyleProfile) -> String {
        let max_len = profile.max_line_length.unwrap_or(100);
        let mut result = Vec::new();

        for line in code.lines() {
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                result.push(String::new());
                continue;
            }

            // Randomly break long lines
            if trimmed.len() > max_len && rand::random::<f32>() < self.config.entropy_level {
                let break_point = max_len - (rand::random::<usize>() % 20);
                if break_point < trimmed.len() {
                    let (first, second) = trimmed.split_at(break_point);
                    result.push(first.to_string());
                    result.push(format!("    {}", second.trim()));
                    continue;
                }
            }

            result.push(trimmed.to_string());
        }

        result.join("\n")
    }

    /// Inject comments to appear more human
    fn inject_comments(&self, code: &str, profile: &StyleProfile) -> String {
        let mut result = Vec::new();
        let comment_prefix = match profile.comment_style {
            CommentStyle::InlineOnly | CommentStyle::Both => "//",
            CommentStyle::BlockOnly => "/*",
            CommentStyle::None => return code.to_string(),
        };

        let human_comments = [
            "Note: this could be optimized further",
            "TODO: consider refactoring",
            "FIXME: edge case handling needed",
            "This is a temporary solution",
            "Performance note: O(n) complexity",
            "Keep this in sync with the other implementation",
            "See related code in module X",
            "This might need adjustment based on requirements",
        ];

        for (i, line) in code.lines().enumerate() {
            result.push(line.to_string());

            // Randomly inject comments
            if !line.trim().is_empty()
                && rand::random::<f32>() < (self.config.entropy_level * 0.1)
                && i % 5 == 0
            {
                let comment = human_comments[rand::random::<usize>() % human_comments.len()];
                result.push(format!("{} {}", comment_prefix, comment));
            }
        }

        result.join("\n")
    }

    /// Vary whitespace to appear more human
    fn vary_whitespace(&self, code: &str, profile: &StyleProfile) -> String {
        let mut result = String::new();

        for line in code.lines() {
            let trimmed = line.trim_start();
            let indent = match profile.indentation {
                IndentationStyle::Spaces(n) => " ".repeat(n as usize),
                IndentationStyle::Tabs => "\t".to_string(),
            };

            // Count original indentation level
            let indent_level = line.len() - trimmed.len();
            let new_indent = indent.repeat((indent_level / indent.len()).max(1));

            result.push_str(&new_indent);
            result.push_str(trimmed);
            result.push('\n');
        }

        result.trim_end().to_string()
    }

    /// Rename variables to match naming convention
    fn rename_variables(&self, code: &str, profile: &StyleProfile) -> String {
        if !self.config.rename_variables {
            return code.to_string();
        }

        let mut result = code.to_string();

        match profile.naming_convention {
            NamingConvention::SnakeCase => {
                // Convert camelCase to snake_case
                let re = Regex::new(r"([a-z])([A-Z])").unwrap();
                result = re.replace_all(&result, "${1}_${2}").to_lowercase();
            }
            NamingConvention::CamelCase => {
                // Convert snake_case to camelCase
                let re = Regex::new(r"_([a-z])").unwrap();
                result = re
                    .replace_all(&result, |caps: &regex::Captures| caps[1].to_uppercase())
                    .to_string();
            }
            NamingConvention::PascalCase => {
                // Convert snake_case to PascalCase
                let re = Regex::new(r"(^|_)([a-z])").unwrap();
                result = re
                    .replace_all(&result, |caps: &regex::Captures| caps[2].to_uppercase())
                    .to_string();
            }
            NamingConvention::KebabCase => {
                // Convert snake_case to kebab-case
                result = result.replace("_", "-");
            }
            NamingConvention::Mixed => {
                // Keep original naming
            }
        }

        result
    }

    /// Analyze repository to extract style profile
    fn analyze_repository(&self, repo_path: &Path) -> anyhow::Result<StyleProfile> {
        let repo = Repository::open(repo_path)?;

        // Analyze commit messages
        let mut commit_messages = Vec::new();
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.simplify_first_parent()?;

        for commit_id in revwalk.take(100) {
            if let Ok(commit) = repo.find_commit(commit_id?) {
                if let Some(msg) = commit.message() {
                    commit_messages.push(msg.to_string());
                }
            }
        }

        // Analyze naming conventions from code
        let naming_convention = self.detect_naming_convention(repo_path)?;

        // Analyze comment style
        let comment_style = self.detect_comment_style(repo_path)?;

        // Analyze indentation
        let indentation = self.detect_indentation(repo_path)?;

        // Analyze commit format
        let commit_format = self.analyze_commit_format(&commit_messages);

        // Detect max line length
        let max_line_length = self.detect_max_line_length(repo_path)?;

        Ok(StyleProfile {
            naming_convention,
            comment_style,
            max_line_length,
            indentation,
            commit_format,
        })
    }

    /// Detect naming convention from repository
    fn detect_naming_convention(&self, repo_path: &Path) -> anyhow::Result<NamingConvention> {
        let mut snake_count = 0;
        let mut camel_count = 0;
        let mut pascal_count = 0;

        // Built once here; compiling these per file was the slow part on big repos
        let snake_re = Regex::new(r"\b[a-z][a-z0-9_]*[a-z0-9]\b").unwrap();
        let camel_re = Regex::new(r"\b[a-z][a-zA-Z0-9]*[A-Z][a-zA-Z0-9]*\b").unwrap();
        let pascal_re = Regex::new(r"\b[A-Z][a-zA-Z0-9]*[A-Z][a-zA-Z0-9]*\b").unwrap();

        // Scan common source files
        for entry in walkdir::WalkDir::new(repo_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                path.extension().is_some_and(|ext| {
                    matches!(
                        ext.to_str(),
                        Some("rs" | "py" | "js" | "ts" | "go" | "cpp" | "c")
                    )
                })
            })
        {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                snake_count += snake_re.find_iter(&content).count();
                camel_count += camel_re.find_iter(&content).count();
                pascal_count += pascal_re.find_iter(&content).count();
            }
        }

        // Determine most common convention
        if snake_count > camel_count && snake_count > pascal_count {
            Ok(NamingConvention::SnakeCase)
        } else if camel_count > pascal_count {
            Ok(NamingConvention::CamelCase)
        } else if pascal_count > 0 {
            Ok(NamingConvention::PascalCase)
        } else {
            Ok(NamingConvention::Mixed)
        }
    }

    /// Detect comment style from repository
    fn detect_comment_style(&self, repo_path: &Path) -> anyhow::Result<CommentStyle> {
        let mut inline_count = 0;
        let mut block_count = 0;
        let mut total_files = 0;

        for entry in walkdir::WalkDir::new(repo_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().is_some_and(|ext| {
                    matches!(
                        ext.to_str(),
                        Some("rs" | "py" | "js" | "ts" | "go" | "cpp" | "c")
                    )
                })
            })
        {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                total_files += 1;
                inline_count += content.matches("//").count();
                block_count += content.matches("/*").count();
            }
        }

        if total_files == 0 {
            return Ok(CommentStyle::None);
        }

        let avg_inline = inline_count as f32 / total_files as f32;
        let avg_block = block_count as f32 / total_files as f32;

        if avg_inline > 0.5 && avg_block > 0.5 {
            Ok(CommentStyle::Both)
        } else if avg_inline > 0.5 {
            Ok(CommentStyle::InlineOnly)
        } else if avg_block > 0.5 {
            Ok(CommentStyle::BlockOnly)
        } else {
            Ok(CommentStyle::None)
        }
    }

    /// Detect indentation style from repository
    fn detect_indentation(&self, repo_path: &Path) -> anyhow::Result<IndentationStyle> {
        let mut space_count = 0;
        let mut tab_count = 0;
        let mut space_sizes = Vec::new();

        for entry in walkdir::WalkDir::new(repo_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().is_some_and(|ext| {
                    matches!(
                        ext.to_str(),
                        Some("rs" | "py" | "js" | "ts" | "go" | "cpp" | "c")
                    )
                })
            })
        {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                for line in content.lines() {
                    let leading_spaces = line.len() - line.trim_start().len();
                    if leading_spaces > 0 {
                        if line.starts_with('\t') {
                            tab_count += 1;
                        } else {
                            space_count += 1;
                            space_sizes.push(leading_spaces);
                        }
                    }
                }
            }
        }

        if tab_count > space_count {
            Ok(IndentationStyle::Tabs)
        } else if !space_sizes.is_empty() {
            // Find most common indentation size
            let mut size_counts = HashMap::new();
            for size in space_sizes {
                *size_counts.entry(size).or_insert(0) += 1;
            }

            let most_common = size_counts
                .iter()
                .max_by_key(|&(_, count)| count)
                .map(|(&size, _)| size as u8)
                .unwrap_or(4);

            Ok(IndentationStyle::Spaces(most_common))
        } else {
            Ok(IndentationStyle::Spaces(4))
        }
    }

    /// Analyze commit message format
    fn analyze_commit_format(&self, messages: &[String]) -> CommitFormat {
        if messages.is_empty() {
            return CommitFormat {
                conventional: false,
                subject_length: Some(50),
                has_body: false,
            };
        }

        let mut conventional_count = 0;
        let mut subject_lengths = Vec::new();
        let mut body_count = 0;

        let conventional_re = Regex::new(
            r"^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?:",
        )
        .unwrap();

        for msg in messages {
            let lines: Vec<&str> = msg.lines().collect();

            if !lines.is_empty() {
                let subject = lines[0];
                subject_lengths.push(subject.len());

                if conventional_re.is_match(subject) {
                    conventional_count += 1;
                }

                if lines.len() > 1 {
                    body_count += 1;
                }
            }
        }

        let avg_subject_length = if !subject_lengths.is_empty() {
            let sum: usize = subject_lengths.iter().sum();
            Some(sum / subject_lengths.len())
        } else {
            Some(50)
        };

        let conventional = (conventional_count as f32 / messages.len() as f32) > 0.5;
        let has_body = (body_count as f32 / messages.len() as f32) > 0.3;

        CommitFormat {
            conventional,
            subject_length: avg_subject_length,
            has_body,
        }
    }

    /// Detect maximum line length from repository
    fn detect_max_line_length(&self, repo_path: &Path) -> anyhow::Result<Option<usize>> {
        let mut line_lengths = Vec::new();

        for entry in walkdir::WalkDir::new(repo_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().is_some_and(|ext| {
                    matches!(
                        ext.to_str(),
                        Some("rs" | "py" | "js" | "ts" | "go" | "cpp" | "c")
                    )
                })
            })
        {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                for line in content.lines() {
                    let len = line.trim_end().len();
                    if len > 0 && len < 200 {
                        line_lengths.push(len);
                    }
                }
            }
        }

        if line_lengths.is_empty() {
            return Ok(None);
        }

        // Use 95th percentile as max line length
        line_lengths.sort();
        let percentile = (line_lengths.len() as f32 * 0.95) as usize;
        Ok(Some(line_lengths.get(percentile).copied().unwrap_or(100)))
    }
}

impl Humanizer for ApexHumanizer {
    fn humanize(&self, code: &CodeBlock, profile: &StyleProfile) -> anyhow::Result<String> {
        let mut result = code.content.clone();

        // Apply variable renaming
        result = self.rename_variables(&result, profile);

        // Apply entropy injection
        result = self.apply_entropy(&result, profile);

        Ok(result)
    }

    fn learn_style(&self, repo_path: &Path) -> anyhow::Result<StyleProfile> {
        // Check cache first
        if let Some(cached) = self.style_cache.get(repo_path) {
            return Ok(cached.clone());
        }

        // Analyze repository
        let profile = self.analyze_repository(repo_path)?;

        // Cache the result
        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humanizer_creation() {
        let humanizer = ApexHumanizer::new();
        assert_eq!(humanizer.config.entropy_level, 0.5);
    }

    #[test]
    fn test_entropy_application() {
        let humanizer = ApexHumanizer::with_config(HumanizerConfig {
            entropy_level: 0.8,
            ..Default::default()
        });

        let profile = StyleProfile {
            naming_convention: NamingConvention::SnakeCase,
            comment_style: CommentStyle::InlineOnly,
            max_line_length: Some(80),
            indentation: IndentationStyle::Spaces(4),
            commit_format: CommitFormat {
                conventional: false,
                subject_length: Some(50),
                has_body: false,
            },
        };

        let code = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let result = humanizer.apply_entropy(code, &profile);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_variable_renaming() {
        let humanizer = ApexHumanizer::with_config(HumanizerConfig {
            rename_variables: true,
            ..Default::default()
        });

        let profile = StyleProfile {
            naming_convention: NamingConvention::SnakeCase,
            comment_style: CommentStyle::InlineOnly,
            max_line_length: Some(100),
            indentation: IndentationStyle::Spaces(4),
            commit_format: CommitFormat {
                conventional: false,
                subject_length: Some(50),
                has_body: false,
            },
        };

        let code = "let myVariable = 42;";
        let result = humanizer.rename_variables(code, &profile);
        assert!(result.contains("my_variable"));
    }
}
