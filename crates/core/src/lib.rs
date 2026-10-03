//! ApexCode Core Library
//!
//! This crate provides the foundational types, traits, and configuration
//! for the ApexCode adversarial stylometry framework.
//!
//! Built with ❤️ by Mohamed Alieu Jagitay

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a block of code with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    /// The file path
    pub path: PathBuf,
    /// The programming language
    pub language: Language,
    /// The source code content
    pub content: String,
    /// Line numbers (start, end)
    pub line_range: (usize, usize),
}

/// Supported programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Cpp,
    C,
    Unknown,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Language::Rust,
            "py" => Language::Python,
            "js" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "mts" | "cts" => Language::TypeScript,
            "go" => Language::Go,
            "cpp" | "cc" | "cxx" => Language::Cpp,
            "c" | "h" => Language::C,
            _ => Language::Unknown,
        }
    }

    pub fn extension(&self) -> Option<&'static str> {
        match self {
            Language::Rust => Some("rs"),
            Language::Python => Some("py"),
            Language::JavaScript => Some("js"),
            Language::TypeScript => Some("ts"),
            Language::Go => Some("go"),
            Language::Cpp => Some("cpp"),
            Language::C => Some("c"),
            Language::Unknown => None,
        }
    }
}

/// Represents a commit message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMessage {
    /// The full commit message
    pub message: String,
    /// The subject line (first line)
    pub subject: String,
    /// The body (everything after the first line)
    pub body: Option<String>,
}

/// Style profile extracted from repository analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleProfile {
    /// Naming convention preference
    pub naming_convention: NamingConvention,
    /// Comment style preference
    pub comment_style: CommentStyle,
    /// Line length preference
    pub max_line_length: Option<usize>,
    /// Indentation style
    pub indentation: IndentationStyle,
    /// Commit message format
    pub commit_format: CommitFormat,
}

/// Naming convention preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NamingConvention {
    SnakeCase,
    CamelCase,
    PascalCase,
    KebabCase,
    Mixed,
}

/// Comment style preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommentStyle {
    None,
    InlineOnly,
    BlockOnly,
    Both,
}

/// Indentation style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndentationStyle {
    Spaces(u8),
    Tabs,
}

/// Commit message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitFormat {
    /// Whether to use conventional commits
    pub conventional: bool,
    /// Typical subject length
    pub subject_length: Option<usize>,
    /// Whether body is typically present
    pub has_body: bool,
}

/// Stealth score for code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthScore {
    /// Overall stealth score (0.0 = obviously AI, 1.0 = definitely human)
    pub overall: f32,
    /// AI detection probability (inverse of stealth)
    pub ai_probability: f32,
    /// Pattern-based score
    pub pattern_score: f32,
    /// Style match score
    pub style_score: f32,
}

impl StealthScore {
    pub fn new(ai_probability: f32, pattern_score: f32, style_score: f32) -> Self {
        let overall = 1.0 - (ai_probability * 0.5 + pattern_score * 0.3 + style_score * 0.2);
        Self {
            overall: overall.max(0.0).min(1.0),
            ai_probability,
            pattern_score,
            style_score,
        }
    }

    /// Returns true if the code is likely AI-generated
    pub fn is_likely_ai(&self, threshold: f32) -> bool {
        self.ai_probability > threshold
    }
}

/// Result of AI detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    /// The stealth score
    pub score: StealthScore,
    /// Detected patterns
    pub patterns: Vec<Pattern>,
    /// Per-section results
    pub sections: Vec<SectionResult>,
}

/// A detected pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Pattern type
    pub pattern_type: PatternType,
    /// Confidence score
    pub confidence: f32,
    /// Location in the code
    pub location: Option<(usize, usize)>,
}

/// Types of patterns that can be detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// Watermark patterns
    Watermark,
    /// Excessive emoji usage
    ExcessiveEmojis,
    /// Uniform comment style
    UniformComments,
    /// Predictable variable naming
    PredictableNaming,
    /// Lack of entropy
    LowEntropy,
    /// Other pattern
    Other(String),
}

/// Result for a specific section of code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionResult {
    /// Section identifier
    pub section_id: String,
    /// Stealth score for this section
    pub score: StealthScore,
    /// Patterns found in this section
    pub patterns: Vec<Pattern>,
}

/// Trait for AI detectors
pub trait Detector: Send + Sync {
    /// Detect AI-generated content in a code block
    fn detect(&self, code: &CodeBlock) -> anyhow::Result<DetectionResult>;

    /// Batch detect multiple code blocks
    fn detect_batch(&self, codes: &[CodeBlock]) -> anyhow::Result<Vec<DetectionResult>> {
        codes.iter().map(|c| self.detect(c)).collect()
    }
}

/// Trait for code humanizers
pub trait Humanizer: Send + Sync {
    /// Humanize a code block based on a style profile
    fn humanize(&self, code: &CodeBlock, profile: &StyleProfile) -> anyhow::Result<String>;

    /// Learn style from a repository
    fn learn_style(&self, repo_path: &PathBuf) -> anyhow::Result<StyleProfile>;
}

/// Trait for jitter engines (temporal obfuscation)
pub trait JitterEngine: Send + Sync {
    /// Apply jitter to a commit operation
    fn apply_jitter(&self, operation: JitterOperation) -> anyhow::Result<JitterSchedule>;

    /// Get the current jitter schedule
    fn current_schedule(&self) -> Option<JitterSchedule>;
}

/// Types of operations that can be jittered
#[derive(Debug, Clone)]
pub enum JitterOperation {
    /// Stage files
    Stage { files: Vec<PathBuf> },
    /// Create a commit
    Commit { message: String },
    /// Push to remote
    Push,
}

/// Schedule for a jittered operation
#[derive(Debug, Clone)]
pub struct JitterSchedule {
    /// When to execute the operation
    pub execute_at: chrono::DateTime<chrono::Utc>,
    /// The operation to execute
    pub operation: JitterOperation,
}

/// Configuration for ApexCode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Detection settings
    pub detection: DetectionConfig,
    /// Humanization settings
    pub humanization: HumanizationConfig,
    /// Jitter settings
    pub jitter: JitterConfig,
    /// API settings
    pub api: ApiConfig,
}

/// Detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionConfig {
    /// AI probability threshold
    pub threshold: f32,
    /// Whether to use local models
    pub use_local: bool,
    /// Whether to use cloud API as fallback
    pub use_cloud_fallback: bool,
}

/// Humanization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanizationConfig {
    /// Whether to auto-humanize on commit
    pub auto_humanize: bool,
    /// Entropy level (0.0 - 1.0)
    pub entropy_level: f32,
}

/// Jitter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitterConfig {
    /// Whether jitter is enabled
    pub enabled: bool,
    /// Minimum delay in seconds
    pub min_delay_secs: u64,
    /// Maximum delay in seconds
    pub max_delay_secs: u64,
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// OpenRouter API key (optional)
    pub openrouter_key: Option<String>,
    /// Anthropic API key (optional)
    pub anthropic_key: Option<String>,
    /// API base URL
    pub base_url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            detection: DetectionConfig {
                threshold: 0.15,
                use_local: true,
                use_cloud_fallback: true,
            },
            humanization: HumanizationConfig {
                auto_humanize: false,
                entropy_level: 0.5,
            },
            jitter: JitterConfig {
                enabled: false,
                min_delay_secs: 60,
                max_delay_secs: 300,
            },
            api: ApiConfig {
                openrouter_key: None,
                anthropic_key: None,
                base_url: None,
            },
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load or create default configuration
    pub fn load_or_default(path: &PathBuf) -> anyhow::Result<Self> {
        if path.exists() {
            Self::load(path)
        } else {
            let config = Self::default();
            config.save(path)?;
            Ok(config)
        }
    }
}
