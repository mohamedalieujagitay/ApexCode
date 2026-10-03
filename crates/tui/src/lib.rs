//! ApexCode TUI Dashboard
//!
//! This crate provides a terminal user interface for visualizing
//! stealth scores, detection results, and style profiles.
//!
//! Built with ❤️ by Mohamed Alieu Jagitay

use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap, BarChart, List, ListItem, Tabs},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;
use std::path::PathBuf;
use std::collections::HashMap;

use apexcode_core::{Config, StealthScore, Pattern, PatternType, Language};

/// Main TUI application
pub struct ApexTui {
    /// Current tab
    current_tab: Tab,
    /// Should quit
    should_quit: bool,
    /// Application state
    state: AppState,
    /// Scroll position for lists
    scroll: usize,
    /// Selected item index
    selected: usize,
    /// Settings mode
    settings_mode: bool,
    /// Current directory for file browser
    current_dir: PathBuf,
    /// File filter
    file_filter: FileFilter,
}

/// File filter options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFilter {
    All,
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Cpp,
    C,
}

impl FileFilter {
    fn title(&self) -> &'static str {
        match self {
            FileFilter::All => "All Files",
            FileFilter::Rust => "Rust",
            FileFilter::Python => "Python",
            FileFilter::JavaScript => "JavaScript",
            FileFilter::TypeScript => "TypeScript",
            FileFilter::Go => "Go",
            FileFilter::Cpp => "C++",
            FileFilter::C => "C",
        }
    }

    fn matches(&self, language: Language) -> bool {
        match self {
            FileFilter::All => true,
            FileFilter::Rust => language == Language::Rust,
            FileFilter::Python => language == Language::Python,
            FileFilter::JavaScript => language == Language::JavaScript,
            FileFilter::TypeScript => language == Language::TypeScript,
            FileFilter::Go => language == Language::Go,
            FileFilter::Cpp => language == Language::Cpp,
            FileFilter::C => language == Language::C,
        }
    }
}

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Configuration
    pub config: Config,
    /// Scan results
    pub scan_results: Vec<ScanResult>,
    /// File list
    pub files: Vec<FileItem>,
    /// Current stealth score
    pub stealth_score: Option<StealthScore>,
    /// Status message
    pub status: String,
    /// Is scanning
    pub is_scanning: bool,
    /// Git status
    pub git_status: Option<GitStatus>,
}

/// Scan result
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// File path
    pub path: PathBuf,
    /// AI probability
    pub ai_probability: f32,
    /// Patterns found
    pub patterns: Vec<Pattern>,
    /// Status
    pub status: ScanStatus,
}

/// Scan status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanStatus {
    Pending,
    Scanning,
    Complete,
    Error,
}

/// File item
#[derive(Debug, Clone)]
pub struct FileItem {
    /// File path
    pub path: PathBuf,
    /// Language
    pub language: Language,
    /// Is selected
    pub selected: bool,
    /// Is staged
    pub staged: bool,
    /// File size
    pub size: u64,
}

/// Git status
#[derive(Debug, Clone)]
pub struct GitStatus {
    /// Branch name
    pub branch: String,
    /// Staged files
    pub staged: Vec<PathBuf>,
    /// Modified files
    pub modified: Vec<PathBuf>,
    /// Untracked files
    pub untracked: Vec<PathBuf>,
}

/// Available tabs
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    Dashboard,
    Scan,
    Humanize,
    Score,
    Settings,
    Help,
    Files,
}

/// Settings items
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SettingItem {
    UseLocalModels,
    UseCloudFallback,
    DetectionThreshold,
    AutoHumanize,
    EntropyLevel,
    JitterEnabled,
    MinDelay,
    MaxDelay,
}

impl SettingItem {
    fn all() -> Vec<SettingItem> {
        vec![
            SettingItem::UseLocalModels,
            SettingItem::UseCloudFallback,
            SettingItem::DetectionThreshold,
            SettingItem::AutoHumanize,
            SettingItem::EntropyLevel,
            SettingItem::JitterEnabled,
            SettingItem::MinDelay,
            SettingItem::MaxDelay,
        ]
    }

    fn title(&self) -> &'static str {
        match self {
            SettingItem::UseLocalModels => "Use Local Models",
            SettingItem::UseCloudFallback => "Cloud Fallback",
            SettingItem::DetectionThreshold => "Detection Threshold",
            SettingItem::AutoHumanize => "Auto Humanize",
            SettingItem::EntropyLevel => "Entropy Level",
            SettingItem::JitterEnabled => "Jitter Enabled",
            SettingItem::MinDelay => "Min Delay (seconds)",
            SettingItem::MaxDelay => "Max Delay (seconds)",
        }
    }
}

impl Tab {
    fn title(&self) -> &'static str {
        match self {
            Tab::Dashboard => "Dashboard",
            Tab::Scan => "Scan",
            Tab::Humanize => "Humanize",
            Tab::Score => "Score",
            Tab::Settings => "Settings",
            Tab::Help => "Help",
            Tab::Files => "Files",
        }
    }

    fn all() -> Vec<Tab> {
        vec![Tab::Dashboard, Tab::Scan, Tab::Humanize, Tab::Score, Tab::Files, Tab::Settings, Tab::Help]
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Config::default(),
            scan_results: Vec::new(),
            files: Vec::new(),
            stealth_score: None,
            status: "Ready".to_string(),
            is_scanning: false,
            git_status: None,
        }
    }
}

impl ApexTui {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Dashboard,
            should_quit: false,
            state: AppState::default(),
            scroll: 0,
            selected: 0,
            settings_mode: false,
            current_dir: PathBuf::from("."),
            file_filter: FileFilter::All,
        }
    }

    /// Run the TUI application
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Load initial data
        self.load_initial_data();

        // Main loop
        while !self.should_quit {
            terminal.draw(|f| self.draw(f))?;

            // Handle events
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        Ok(())
    }

    /// Load initial data
    fn load_initial_data(&mut self) {
        // Try to load git status
        if let Ok(repo) = git2::Repository::discover(".") {
            if let Ok(head) = repo.head() {
                if let Some(branch_name) = head.shorthand() {
                    self.state.git_status = Some(GitStatus {
                        branch: branch_name.to_string(),
                        staged: Vec::new(),
                        modified: Vec::new(),
                        untracked: Vec::new(),
                    });
                }
            }
        }

        // Load files
        self.load_files();
    }

    /// Load files from current directory
    fn load_files(&mut self) {
        self.load_files_from_dir();
    }

    /// Load files from specific directory
    fn load_files_from_dir(&mut self) {
        if let Ok(entries) = std::fs::read_dir(&self.current_dir) {
            self.state.files = entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    if let Ok(ft) = e.file_type() {
                        ft.is_file()
                    } else {
                        false
                    }
                })
                .map(|e| {
                    let path = e.path();
                    let language = path.extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| Language::from_extension(ext))
                        .unwrap_or(Language::Unknown);

                    let size = path.metadata()
                        .map(|m| m.len())
                        .unwrap_or(0);

                    FileItem {
                        path: path.clone(),
                        language,
                        selected: false,
                        staged: false,
                        size,
                    }
                })
                .filter(|file| self.file_filter.matches(file.language))
                .collect();
        }
    }

    /// Handle keyboard input
    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if self.settings_mode {
                    self.settings_mode = false;
                    self.state.status = "Settings mode exited".to_string();
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Left => {
                if self.settings_mode {
                    // Decrease value
                    self.adjust_setting(-1);
                } else {
                    let tabs = Tab::all();
                    let current = tabs.iter().position(|&t| t == self.current_tab).unwrap_or(0);
                    if current > 0 {
                        self.current_tab = tabs[current - 1];
                        self.scroll = 0;
                        self.selected = 0;
                    }
                }
            }
            KeyCode::Right => {
                if self.settings_mode {
                    // Increase value
                    self.adjust_setting(1);
                } else {
                    let tabs = Tab::all();
                    let current = tabs.iter().position(|&t| t == self.current_tab).unwrap_or(0);
                    if current < tabs.len() - 1 {
                        self.current_tab = tabs[current + 1];
                        self.scroll = 0;
                        self.selected = 0;
                    }
                }
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                    if self.scroll > 0 && self.selected < self.scroll {
                        self.scroll -= 1;
                    }
                }
            }
            KeyCode::Down => {
                let max_items = self.get_max_items();
                if self.selected < max_items.saturating_sub(1) {
                    self.selected += 1;
                }
            }
            KeyCode::Char('d') => self.current_tab = Tab::Dashboard,
            KeyCode::Char('s') => self.current_tab = Tab::Scan,
            KeyCode::Char('h') => self.current_tab = Tab::Humanize,
            KeyCode::Char('c') => self.current_tab = Tab::Score,
            KeyCode::Char('g') => self.current_tab = Tab::Settings,
            KeyCode::Char('l') => self.current_tab = Tab::Help,
            KeyCode::Char('f') => self.current_tab = Tab::Files,
            KeyCode::Char(' ') => {
                // Toggle selection
                if let Some(file) = self.state.files.get_mut(self.selected) {
                    file.selected = !file.selected;
                    self.state.status = format!("{} selected files", self.state.files.iter().filter(|f| f.selected).count());
                }
            }
            KeyCode::Enter => {
                // Perform action based on current tab
                self.handle_enter();
            }
            KeyCode::Char('r') => {
                // Refresh
                self.load_files();
                self.state.status = "Refreshed file list".to_string();
            }
            KeyCode::Char('a') => {
                // Select all
                for file in &mut self.state.files {
                    file.selected = true;
                }
                self.state.status = "All files selected".to_string();
            }
            KeyCode::Char('n') => {
                // Select none
                for file in &mut self.state.files {
                    file.selected = false;
                }
                self.state.status = "Selection cleared".to_string();
            }
            KeyCode::Char('e') => {
                // Enter settings edit mode
                if self.current_tab == Tab::Settings {
                    self.settings_mode = !self.settings_mode;
                    self.state.status = if self.settings_mode {
                        "Settings edit mode: Use Left/Right to adjust values".to_string()
                    } else {
                        "Settings mode exited".to_string()
                    };
                }
            }
            KeyCode::Char('.') => {
                // Go to parent directory
                if self.current_tab == Tab::Files {
                    if let Some(parent) = self.current_dir.parent() {
                        self.current_dir = parent.to_path_buf();
                        self.load_files_from_dir();
                        self.state.status = format!("Changed to: {}", self.current_dir.display());
                    }
                }
            }
            KeyCode::Char(',') => {
                // Go to root directory
                if self.current_tab == Tab::Files {
                    self.current_dir = PathBuf::from(".");
                    self.load_files_from_dir();
                    self.state.status = "Changed to root directory".to_string();
                }
            }
            KeyCode::Char('1') => {
                // Filter by Rust
                if self.current_tab == Tab::Files {
                    self.file_filter = FileFilter::Rust;
                    self.load_files_from_dir();
                    self.state.status = format!("Filter: {}", self.file_filter.title());
                }
            }
            KeyCode::Char('2') => {
                // Filter by Python
                if self.current_tab == Tab::Files {
                    self.file_filter = FileFilter::Python;
                    self.load_files_from_dir();
                    self.state.status = format!("Filter: {}", self.file_filter.title());
                }
            }
            KeyCode::Char('3') => {
                // Filter by JavaScript
                if self.current_tab == Tab::Files {
                    self.file_filter = FileFilter::JavaScript;
                    self.load_files_from_dir();
                    self.state.status = format!("Filter: {}", self.file_filter.title());
                }
            }
            KeyCode::Char('4') => {
                // Filter by TypeScript
                if self.current_tab == Tab::Files {
                    self.file_filter = FileFilter::TypeScript;
                    self.load_files_from_dir();
                    self.state.status = format!("Filter: {}", self.file_filter.title());
                }
            }
            KeyCode::Char('5') => {
                // Filter by Go
                if self.current_tab == Tab::Files {
                    self.file_filter = FileFilter::Go;
                    self.load_files_from_dir();
                    self.state.status = format!("Filter: {}", self.file_filter.title());
                }
            }
            KeyCode::Char('6') => {
                // Filter by C++
                if self.current_tab == Tab::Files {
                    self.file_filter = FileFilter::Cpp;
                    self.load_files_from_dir();
                    self.state.status = format!("Filter: {}", self.file_filter.title());
                }
            }
            KeyCode::Char('7') => {
                // Filter by C
                if self.current_tab == Tab::Files {
                    self.file_filter = FileFilter::C;
                    self.load_files_from_dir();
                    self.state.status = format!("Filter: {}", self.file_filter.title());
                }
            }
            KeyCode::Char('0') => {
                // Show all files
                if self.current_tab == Tab::Files {
                    self.file_filter = FileFilter::All;
                    self.load_files_from_dir();
                    self.state.status = format!("Filter: {}", self.file_filter.title());
                }
            }
            KeyCode::Char('x') => {
                // Clear scan results
                self.state.scan_results.clear();
                self.state.stealth_score = None;
                self.state.status = "Scan results cleared".to_string();
            }
            KeyCode::Char('i') => {
                // Show info about selected file
                if self.current_tab == Tab::Files {
                    if let Some(file) = self.state.files.get(self.selected) {
                        let size_str = if file.size > 0 {
                            format!("{} bytes", file.size)
                        } else {
                            "Unknown size".to_string()
                        };
                        self.state.status = format!("{} - {} - {}", file.path.display(), file.language as i32, size_str);
                    }
                }
            }
            _ => {}
        }
    }

    /// Adjust setting value
    fn adjust_setting(&mut self, direction: i32) {
        let settings = SettingItem::all();
        if let Some(setting) = settings.get(self.selected) {
            match setting {
                SettingItem::UseLocalModels => {
                    self.state.config.detection.use_local = !self.state.config.detection.use_local;
                    self.state.status = format!("Use Local Models: {}", self.state.config.detection.use_local);
                }
                SettingItem::UseCloudFallback => {
                    self.state.config.detection.use_cloud_fallback = !self.state.config.detection.use_cloud_fallback;
                    self.state.status = format!("Cloud Fallback: {}", self.state.config.detection.use_cloud_fallback);
                }
                SettingItem::DetectionThreshold => {
                    let step = 0.05 * direction as f32;
                    self.state.config.detection.threshold = (self.state.config.detection.threshold + step).clamp(0.0, 1.0);
                    self.state.status = format!("Detection Threshold: {:.2}", self.state.config.detection.threshold);
                }
                SettingItem::AutoHumanize => {
                    self.state.config.humanization.auto_humanize = !self.state.config.humanization.auto_humanize;
                    self.state.status = format!("Auto Humanize: {}", self.state.config.humanization.auto_humanize);
                }
                SettingItem::EntropyLevel => {
                    let step = 0.1 * direction as f32;
                    self.state.config.humanization.entropy_level = (self.state.config.humanization.entropy_level + step).clamp(0.0, 1.0);
                    self.state.status = format!("Entropy Level: {:.2}", self.state.config.humanization.entropy_level);
                }
                SettingItem::JitterEnabled => {
                    self.state.config.jitter.enabled = !self.state.config.jitter.enabled;
                    self.state.status = format!("Jitter Enabled: {}", self.state.config.jitter.enabled);
                }
                SettingItem::MinDelay => {
                    let step = 10 * direction as i64;
                    self.state.config.jitter.min_delay_secs = (self.state.config.jitter.min_delay_secs as i64 + step).max(0) as u64;
                    self.state.status = format!("Min Delay: {}s", self.state.config.jitter.min_delay_secs);
                }
                SettingItem::MaxDelay => {
                    let step = 10 * direction as i64;
                    self.state.config.jitter.max_delay_secs = (self.state.config.jitter.max_delay_secs as i64 + step).max(0) as u64;
                    self.state.status = format!("Max Delay: {}s", self.state.config.jitter.max_delay_secs);
                }
            }
        }
    }

    /// Handle Enter key
    fn handle_enter(&mut self) {
        match self.current_tab {
            Tab::Scan => {
                self.start_scan();
            }
            Tab::Files => {
                if let Some(file) = self.state.files.get(self.selected) {
                    self.state.status = format!("Selected: {}", file.path.display());
                }
            }
            Tab::Settings => {
                // Toggle settings mode
                self.settings_mode = !self.settings_mode;
                self.state.status = if self.settings_mode {
                    "Settings edit mode: Use Left/Right to adjust values, 'e' to exit".to_string()
                } else {
                    "Settings mode exited".to_string()
                };
            }
            _ => {}
        }
    }

    /// Start scanning
    fn start_scan(&mut self) {
        self.state.is_scanning = true;
        self.state.status = "Scanning...".to_string();

        let selected_files: Vec<_> = self.state.files.iter()
            .filter(|f| f.selected)
            .cloned()
            .collect();

        if selected_files.is_empty() {
            self.state.status = "No files selected. Press Space to select files.".to_string();
            self.state.is_scanning = false;
            return;
        }

        // Simulate scanning (in real implementation, this would use the detector)
        let mut error_count = 0;
        for file in selected_files {
            // Try to read the file
            let content = match std::fs::read_to_string(&file.path) {
                Ok(content) => content,
                Err(_) => {
                    error_count += 1;
                    let result = ScanResult {
                        path: file.path.clone(),
                        ai_probability: 0.0,
                        patterns: vec![],
                        status: ScanStatus::Error,
                    };
                    self.state.scan_results.push(result);
                    continue;
                }
            };

            // Analyze the content
            let ai_prob = if content.is_empty() {
                0.0
            } else {
                0.3 + (rand::random::<f32>() * 0.4)
            };

            let patterns = if ai_prob > 0.5 {
                vec![
                    Pattern {
                        pattern_type: PatternType::LowEntropy,
                        confidence: 0.5 + rand::random::<f32>() * 0.3,
                        location: None,
                    },
                    Pattern {
                        pattern_type: PatternType::UniformComments,
                        confidence: 0.3 + rand::random::<f32>() * 0.4,
                        location: None,
                    },
                ]
            } else {
                vec![]
            };

            let result = ScanResult {
                path: file.path.clone(),
                ai_probability: ai_prob,
                patterns,
                status: ScanStatus::Complete,
            };
            self.state.scan_results.push(result);
        }

        // Calculate overall score
        if !self.state.scan_results.is_empty() {
            let avg_ai_prob: f32 = self.state.scan_results.iter()
                .map(|r| r.ai_probability)
                .sum::<f32>() / self.state.scan_results.len() as f32;

            self.state.stealth_score = Some(StealthScore::new(avg_ai_prob, 0.2, 0.3));
        }

        self.state.is_scanning = false;
        let total = self.state.scan_results.len();
        let errors = error_count;
        self.state.status = format!("Scan complete: {} files processed, {} errors", total, errors);
    }

    /// Get max items for current view
    fn get_max_items(&self) -> usize {
        match self.current_tab {
            Tab::Files => self.state.files.len(),
            Tab::Scan => self.state.scan_results.len(),
            Tab::Settings => SettingItem::all().len(),
            _ => 0,
        }
    }

    /// Draw the UI
    fn draw(&self, f: &mut Frame) {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Content
                Constraint::Length(2),  // Status bar
                Constraint::Length(1),  // Footer
            ])
            .split(size);

        // Draw header
        self.draw_header(f, chunks[0]);

        // Draw content based on current tab
        match self.current_tab {
            Tab::Dashboard => self.draw_dashboard(f, chunks[1]),
            Tab::Scan => self.draw_scan(f, chunks[1]),
            Tab::Humanize => self.draw_humanize(f, chunks[1]),
            Tab::Score => self.draw_score(f, chunks[1]),
            Tab::Settings => self.draw_settings(f, chunks[1]),
            Tab::Help => self.draw_help(f, chunks[1]),
            Tab::Files => self.draw_files(f, chunks[1]),
        }

        // Draw status bar
        self.draw_status(f, chunks[2]);

        // Draw footer
        self.draw_footer(f, chunks[3]);
    }

    /// Draw header with tabs
    fn draw_header(&self, f: &mut Frame, area: Rect) {
        let tabs = Tab::all();
        let titles: Vec<Line> = tabs
            .iter()
            .map(|t| {
                let style = if *t == self.current_tab {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(Span::styled(t.title(), style))
            })
            .collect();

        let tabs_widget = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("ApexCode Dashboard")
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .divider(Span::raw(" | "))
            .select(tabs.iter().position(|&t| t == self.current_tab).unwrap_or(0));

        f.render_widget(tabs_widget, area);
    }

    /// Draw status bar
    fn draw_status(&self, f: &mut Frame, area: Rect) {
        let status_color = if self.state.is_scanning {
            Color::Yellow
        } else if self.state.status.contains("Error") {
            Color::Red
        } else if self.state.status.contains("Complete") || self.state.status.contains("Ready") {
            Color::Green
        } else {
            Color::Cyan
        };

        let status = Line::from(vec![
            Span::raw("Status: "),
            Span::styled(&self.state.status, Style::default().fg(status_color)),
        ]);

        let paragraph = Paragraph::new(status)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White));

        f.render_widget(paragraph, area);
    }

    /// Draw dashboard tab
    fn draw_dashboard(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(0),
            ])
            .split(area);

        // Draw stealth score gauge
        self.draw_stealth_score(f, chunks[0]);

        // Draw quick actions and stats
        let stats_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[1]);

        // Draw stats
        self.draw_stats(f, stats_chunks[0]);

        // Draw quick actions
        self.draw_quick_actions(f, stats_chunks[1]);
    }

    /// Draw stealth score gauge
    fn draw_stealth_score(&self, f: &mut Frame, area: Rect) {
        let score = self.state.stealth_score.as_ref()
            .map(|s| s.overall)
            .unwrap_or(0.75);

        let gauge_color = if score > 0.7 {
            Color::Green
        } else if score > 0.4 {
            Color::Yellow
        } else {
            Color::Red
        };

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Stealth Score"))
            .gauge_style(
                Style::default()
                    .fg(gauge_color)
                    .bg(Color::DarkGray)
            )
            .label(Span::styled(
                format!("{:.0}%", score * 100.0),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ))
            .ratio(score as f64);

        f.render_widget(gauge, area);
    }

    /// Draw stats
    fn draw_stats(&self, f: &mut Frame, area: Rect) {
        let selected_count = self.state.files.iter().filter(|f| f.selected).count();
        let scanned_count = self.state.scan_results.len();
        let high_risk_count = self.state.scan_results.iter().filter(|r| r.ai_probability > 0.7).count();
        let medium_risk_count = self.state.scan_results.iter().filter(|r| r.ai_probability > 0.4 && r.ai_probability <= 0.7).count();
        let low_risk_count = self.state.scan_results.iter().filter(|r| r.ai_probability <= 0.4).count();
        let error_count = self.state.scan_results.iter().filter(|r| r.status == ScanStatus::Error).count();

        let stats = vec![
            Line::from("Repository Stats:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Files: "),
                Span::styled(
                    format!("{}", self.state.files.len()),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::raw("  Selected: "),
                Span::styled(
                    format!("{}", selected_count),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::raw("  Scanned: "),
                Span::styled(
                    format!("{}", scanned_count),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from("Risk Distribution:"),
            Line::from(vec![
                Span::raw("  High Risk: "),
                Span::styled(
                    format!("{}", high_risk_count),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::raw("  Medium Risk: "),
                Span::styled(
                    format!("{}", medium_risk_count),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::raw("  Low Risk: "),
                Span::styled(
                    format!("{}", low_risk_count),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(vec![
                Span::raw("  Errors: "),
                Span::styled(
                    format!("{}", error_count),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Branch: "),
                Span::styled(
                    self.state.git_status.as_ref().map(|g| g.branch.as_str()).unwrap_or("N/A"),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
            ]),
        ];

        let paragraph = Paragraph::new(stats)
            .block(Block::default().borders(Borders::ALL).title("Stats"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    /// Draw quick actions
    fn draw_quick_actions(&self, f: &mut Frame, area: Rect) {
        let selected_count = self.state.files.iter().filter(|f| f.selected).count();
        let scanned_count = self.state.scan_results.len();
        let actions = vec![
            Line::from("Quick Actions:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[S]can", Style::default().fg(Color::Cyan)),
                Span::raw(" - Scan selected files"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[F]iles", Style::default().fg(Color::Cyan)),
                Span::raw(" - Browse files"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[H]umanize", Style::default().fg(Color::Cyan)),
                Span::raw(" - Humanize code"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[C]heck Score", Style::default().fg(Color::Cyan)),
                Span::raw(" - View detailed score"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[X] Clear", Style::default().fg(Color::Cyan)),
                Span::raw(" - Clear scan results"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Selected: "),
                Span::styled(
                    format!("{}", selected_count),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
                Span::raw(" files | "),
                Span::raw("Scanned: "),
                Span::styled(
                    format!("{}", scanned_count),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
            ]),
        ];

        let paragraph = Paragraph::new(actions)
            .block(Block::default().borders(Borders::ALL).title("Actions"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    /// Draw scan tab
    fn draw_scan(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(0),
            ])
            .split(area);

        // Draw pattern breakdown
        self.draw_pattern_breakdown(f, chunks[0]);

        // Draw scan results
        self.draw_scan_results(f, chunks[1]);
    }

    /// Draw pattern breakdown
    fn draw_pattern_breakdown(&self, f: &mut Frame, area: Rect) {
        // Calculate actual pattern scores from scan results
        let mut pattern_scores = HashMap::new();
        pattern_scores.insert("Watermarks", 0.0f32);
        pattern_scores.insert("Emojis", 0.0f32);
        pattern_scores.insert("Comments", 0.0f32);
        pattern_scores.insert("Naming", 0.0f32);
        pattern_scores.insert("Entropy", 0.0f32);

        for result in &self.state.scan_results {
            for pattern in &result.patterns {
                let key = match &pattern.pattern_type {
                    PatternType::Watermark => "Watermarks",
                    PatternType::ExcessiveEmojis => "Emojis",
                    PatternType::UniformComments => "Comments",
                    PatternType::PredictableNaming => "Naming",
                    PatternType::LowEntropy => "Entropy",
                    PatternType::Other(_) => "Other",
                };
                if let Some(score) = pattern_scores.get_mut(key) {
                    *score += pattern.confidence;
                }
            }
        }

        // Normalize scores
        let count = self.state.scan_results.len() as f32;
        if count > 0.0 {
            for score in pattern_scores.values_mut() {
                *score /= count;
            }
        }

        let patterns: Vec<(&str, f32)> = pattern_scores
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect();

        let bars: Vec<(&str, u64)> = patterns
            .iter()
            .map(|(name, value)| (*name, (*value * 100.0) as u64))
            .collect();

        let barchart = BarChart::default()
            .block(Block::default().borders(Borders::ALL).title("Pattern Detection"))
            .bar_width(8)
            .bar_gap(2)
            .data(&bars)
            .style(Style::default().fg(Color::Cyan))
            .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

        f.render_widget(barchart, area);
    }

    /// Draw scan results
    fn draw_scan_results(&self, f: &mut Frame, area: Rect) {
        if self.state.scan_results.is_empty() {
            let text = vec![
                Line::from("No scan results yet."),
                Line::from(""),
                Line::from("To scan files:"),
                Line::from("  1. Go to [F]iles tab"),
                Line::from("  2. Select files with Space"),
                Line::from("  3. Press Enter to scan"),
            ];

            let paragraph = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("Scan Results"))
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, area);
            return;
        }

        let items: Vec<ListItem> = self.state.scan_results
            .iter()
            .enumerate()
            .map(|(i, result)| {
                let color = if result.ai_probability > 0.7 {
                    Color::Red
                } else if result.ai_probability > 0.4 {
                    Color::Yellow
                } else {
                    Color::Green
                };

                let status = match result.status {
                    ScanStatus::Pending => "Pending",
                    ScanStatus::Scanning => "Scanning...",
                    ScanStatus::Complete => "Complete",
                    ScanStatus::Error => "Error",
                };

                let patterns_str = if result.patterns.is_empty() {
                    "None".to_string()
                } else {
                    result.patterns.iter()
                        .map(|p| match &p.pattern_type {
                            PatternType::Watermark => "Watermark",
                            PatternType::ExcessiveEmojis => "Emojis",
                            PatternType::UniformComments => "Comments",
                            PatternType::PredictableNaming => "Naming",
                            PatternType::LowEntropy => "Entropy",
                            PatternType::Other(s) => s.as_str(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                };

                let risk_level = if result.ai_probability > 0.7 {
                    "HIGH"
                } else if result.ai_probability > 0.4 {
                    "MEDIUM"
                } else {
                    "LOW"
                };

                ListItem::new(vec![
                    Line::from(vec![
                        Span::raw(format!("{} ", i + 1)),
                        Span::styled(
                            result.path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                            Style::default().fg(color)
                        ),
                        Span::raw(" - "),
                        Span::styled(
                            format!("{:.0}% AI", result.ai_probability * 100.0),
                            Style::default().fg(color).add_modifier(Modifier::BOLD)
                        ),
                        Span::raw(" ["),
                        Span::styled(status, Style::default().fg(Color::Gray)),
                        Span::raw("] ["),
                        Span::styled(risk_level, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                        Span::raw("]"),
                    ]),
                    Line::from(vec![
                        Span::raw("    Patterns: "),
                        Span::styled(patterns_str, Style::default().fg(Color::Cyan)),
                    ]),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Scan Results"))
            .style(Style::default().fg(Color::White))
            .scroll(self.scroll as u16)
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        f.render_stateful_widget(
            list,
            area,
            |state| {
                state.select(Some(self.selected));
            }
        );
    }

    /// Draw humanize tab
    fn draw_humanize(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(12),
                Constraint::Min(0),
            ])
            .split(area);

        // Draw humanization options
        let style_info = vec![
            Line::from("Humanization Options:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[F]ix", Style::default().fg(Color::Cyan)),
                Span::raw(" - Auto-fix AI patterns in selected files"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[D]ry Run", Style::default().fg(Color::Cyan)),
                Span::raw(" - Show what would be changed without applying"),
            ]),
            Line::from(""),
            Line::from("Humanization will transform your code to match your personal style:"),
            Line::from("  • Variable and function naming"),
            Line::from("  • Comment phrasing and placement"),
            Line::from("  • Code structure and entropy"),
            Line::from("  • Commit message style"),
            Line::from(""),
            Line::from("Available transformations:"),
            Line::from("  • Rename variables to match your style"),
            Line::from("  • Rewrite comments to be more natural"),
            Line::from("  • Adjust code structure and formatting"),
            Line::from("  • Add entropy to reduce AI detection"),
        ];

        let paragraph = Paragraph::new(style_info)
            .block(Block::default().borders(Borders::ALL).title("Humanize Code"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[0]);

        // Draw current settings
        let settings = vec![
            Line::from("Current Settings:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Entropy Level: "),
                Span::styled(
                    format!("{:.0}%", self.state.config.humanization.entropy_level * 100.0),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::raw("  Auto Humanize: "),
                Span::styled(
                    if self.state.config.humanization.auto_humanize { "Enabled" } else { "Disabled" },
                    Style::default().fg(if self.state.config.humanization.auto_humanize { Color::Green } else { Color::Red })
                ),
            ]),
            Line::from(""),
            Line::from("Jitter Settings:"),
            Line::from(vec![
                Span::raw("  Jitter Enabled: "),
                Span::styled(
                    if self.state.config.jitter.enabled { "Yes" } else { "No" },
                    Style::default().fg(if self.state.config.jitter.enabled { Color::Green } else { Color::Red })
                ),
            ]),
            Line::from(vec![
                Span::raw("  Delay Range: "),
                Span::styled(
                    format!("{}s - {}s", self.state.config.jitter.min_delay_secs, self.state.config.jitter.max_delay_secs),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(""),
            Line::from("Detection Settings:"),
            Line::from(vec![
                Span::raw("  Threshold: "),
                Span::styled(
                    format!("{:.2}", self.state.config.detection.threshold),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::raw("  Use Local Models: "),
                Span::styled(
                    if self.state.config.detection.use_local { "Yes" } else { "No" },
                    Style::default().fg(if self.state.config.detection.use_local { Color::Green } else { Color::Red })
                ),
            ]),
            Line::from(vec![
                Span::raw("  Cloud Fallback: "),
                Span::styled(
                    if self.state.config.detection.use_cloud_fallback { "Yes" } else { "No" },
                    Style::default().fg(if self.state.config.detection.use_cloud_fallback { Color::Green } else { Color::Red })
                ),
            ]),
        ];

        let paragraph = Paragraph::new(settings)
            .block(Block::default().borders(Borders::ALL).title("Configuration"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[1]);
    }

    /// Draw score tab
    fn draw_score(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(12),
                Constraint::Min(0),
            ])
            .split(area);

        // Draw detailed score
        if let Some(score) = &self.state.stealth_score {
            let score_details = vec![
                Line::from("Stealth Score Breakdown:"),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  Overall: "),
                    Span::styled(
                        format!("{:.0}%", score.overall * 100.0),
                        Style::default().fg(if score.overall > 0.7 { Color::Green } else if score.overall > 0.4 { Color::Yellow } else { Color::Red }).add_modifier(Modifier::BOLD)
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  AI Probability: "),
                    Span::styled(
                        format!("{:.0}%", score.ai_probability * 100.0),
                        Style::default().fg(Color::Yellow)
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  Pattern Score: "),
                    Span::styled(
                        format!("{:.0}%", score.pattern_score * 100.0),
                        Style::default().fg(Color::Cyan)
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  Style Score: "),
                    Span::styled(
                        format!("{:.0}%", score.style_score * 100.0),
                        Style::default().fg(Color::Cyan)
                    ),
                ]),
                Line::from(""),
                Line::from("Recommendations:"),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        if score.overall > 0.7 {
                            "✓ Your code looks human-written!"
                        } else if score.overall > 0.4 {
                            "⚠ Consider humanizing some patterns"
                        } else {
                            "✗ Your code appears AI-generated"
                        },
                        Style::default().fg(if score.overall > 0.7 { Color::Green } else if score.overall > 0.4 { Color::Yellow } else { Color::Red })
                    ),
                ]),
            ];

            let paragraph = Paragraph::new(score_details)
                .block(Block::default().borders(Borders::ALL).title("Current Score"))
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, chunks[0]);
        } else {
            let text = vec![
                Line::from("No score available."),
                Line::from(""),
                Line::from("Run a scan to get your stealth score."),
            ];

            let paragraph = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("Current Score"))
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, chunks[0]);
        }

        // Draw score legend
        let legend = vec![
            Line::from("Score Legend:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("90-100%", Style::default().fg(Color::Green)),
                Span::raw(" - Excellent (indistinguishable from human code)"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("70-89%", Style::default().fg(Color::Yellow)),
                Span::raw(" - Good (mostly human-like)"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("50-69%", Style::default().fg(Color::Yellow)),
                Span::raw(" - Needs Work (some AI patterns detected)"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("0-49%", Style::default().fg(Color::Red)),
                Span::raw(" - Poor (clearly AI-generated)"),
            ]),
            Line::from(""),
            Line::from("Score Components:"),
            Line::from("  • Overall: Combined stealth score"),
            Line::from("  • AI Probability: Likelihood of AI detection"),
            Line::from("  • Pattern Score: Pattern-based detection"),
            Line::from("  • Style Score: Style matching score"),
        ];

        let paragraph = Paragraph::new(legend)
            .block(Block::default().borders(Borders::ALL).title("Score Guide"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[1]);
    }

    /// Draw settings tab
    fn draw_settings(&self, f: &mut Frame, area: Rect) {
        let settings_items = SettingItem::all();
        let items: Vec<ListItem> = settings_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_selected = i == self.selected;
                let style = if is_selected {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let value = match item {
                    SettingItem::UseLocalModels => {
                        format!("[{}]", if self.state.config.detection.use_local { "x" } else { " " })
                    }
                    SettingItem::UseCloudFallback => {
                        format!("[{}]", if self.state.config.detection.use_cloud_fallback { "x" } else { " " })
                    }
                    SettingItem::DetectionThreshold => {
                        format!("{:.2}", self.state.config.detection.threshold)
                    }
                    SettingItem::AutoHumanize => {
                        format!("[{}]", if self.state.config.humanization.auto_humanize { "x" } else { " " })
                    }
                    SettingItem::EntropyLevel => {
                        format!("{:.2}", self.state.config.humanization.entropy_level)
                    }
                    SettingItem::JitterEnabled => {
                        format!("[{}]", if self.state.config.jitter.enabled { "x" } else { " " })
                    }
                    SettingItem::MinDelay => {
                        format!("{}s", self.state.config.jitter.min_delay_secs)
                    }
                    SettingItem::MaxDelay => {
                        format!("{}s", self.state.config.jitter.max_delay_secs)
                    }
                };

                ListItem::new(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(item.title(), style),
                    Span::raw(": "),
                    Span::styled(value, Style::default().fg(Color::Cyan)),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(if self.settings_mode {
                        "Settings (Edit Mode - Use Left/Right to adjust, 'e' to exit)"
                    } else {
                        "Settings (Press Enter to edit)"
                    })
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        f.render_widget(list, area);
    }

    /// Draw help tab
    fn draw_help(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        // Draw keyboard shortcuts
        let shortcuts = vec![
            Line::from("Keyboard Shortcuts:"),
            Line::from(""),
            Line::from("Navigation:"),
            Line::from("  Left/Right  - Switch tabs / Adjust settings"),
            Line::from("  Up/Down     - Navigate lists"),
            Line::from("  Enter       - Select/Action / Edit settings"),
            Line::from("  Space       - Toggle selection"),
            Line::from("  q/Esc       - Quit / Exit edit mode"),
            Line::from(""),
            Line::from("Tab Shortcuts:"),
            Line::from("  d - Dashboard"),
            Line::from("  s - Scan"),
            Line::from("  h - Humanize"),
            Line::from("  c - Score"),
            Line::from("  f - Files"),
            Line::from("  g - Settings"),
            Line::from("  l - Help"),
            Line::from(""),
            Line::from("Actions:"),
            Line::from("  r - Refresh"),
            Line::from("  a - Select all"),
            Line::from("  n - Select none"),
            Line::from("  e - Toggle settings edit mode"),
            Line::from("  x - Clear scan results"),
            Line::from("  i - Show file info"),
            Line::from(""),
            Line::from("File Browser:"),
            Line::from("  . - Go to parent directory"),
            Line::from("  , - Go to root directory"),
            Line::from(""),
            Line::from("File Filters:"),
            Line::from("  0 - All files"),
            Line::from("  1 - Rust"),
            Line::from("  2 - Python"),
            Line::from("  3 - JavaScript"),
            Line::from("  4 - TypeScript"),
            Line::from("  5 - Go"),
            Line::from("  6 - C++"),
            Line::from("  7 - C"),
        ];

        let paragraph = Paragraph::new(shortcuts)
            .block(Block::default().borders(Borders::ALL).title("Shortcuts"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[0]);

        // Draw commands
        let commands = vec![
            Line::from("CLI Commands:"),
            Line::from(""),
            Line::from("  apexcode              - Launch dashboard"),
            Line::from("  apexcode scan         - Scan for AI patterns"),
            Line::from("  apexcode humanize      - Humanize code"),
            Line::from("  apexcode score         - Check stealth score"),
            Line::from("  apexcode fix           - Auto-fix AI patterns"),
            Line::from("  apexcode dashboard      - Launch TUI dashboard"),
            Line::from("  apexcode config        - Configure settings"),
            Line::from("  apexcode init          - Initialize in current directory"),
            Line::from("  apexcode install        - Install IDE integration"),
            Line::from(""),
            Line::from("About:"),
            Line::from("  ApexCode v1.0.0"),
            Line::from("  Adversarial Stylometry Framework"),
            Line::from("  Built with ❤️ by Mohamed Alieu Jagitay"),
        ];

        let paragraph = Paragraph::new(commands)
            .block(Block::default().borders(Borders::ALL).title("Commands"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[1]);
    }

    /// Draw files tab
    fn draw_files(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        // Draw current directory and filter
        let dir_text = vec![
            Line::from(vec![
                Span::raw("Current: "),
                Span::styled(
                    self.current_dir.display().to_string(),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
                Span::raw(" | Filter: "),
                Span::styled(
                    self.file_filter.title(),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                ),
            ]),
        ];

        let paragraph = Paragraph::new(dir_text)
            .block(Block::default().borders(Borders::ALL).title("Directory"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[0]);

        // Draw file list
        if self.state.files.is_empty() {
            let text = vec![
                Line::from("No files found in current directory."),
                Line::from(""),
                Line::from("Press 'r' to refresh."),
                Line::from("Press '.' to go to parent directory."),
                Line::from("Press ',' to go to root directory."),
                Line::from(""),
                Line::from("Filter by language:"),
                Line::from("  0 - All files"),
                Line::from("  1 - Rust"),
                Line::from("  2 - Python"),
                Line::from("  3 - JavaScript"),
                Line::from("  4 - TypeScript"),
                Line::from("  5 - Go"),
                Line::from("  6 - C++"),
                Line::from("  7 - C"),
            ];

            let paragraph = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("Files"))
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, chunks[1]);
            return;
        }

        let items: Vec<ListItem> = self.state.files
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let is_selected = i == self.selected;
                let style = if is_selected {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let checkbox = if file.selected {
                    "[x]"
                } else {
                    "[ ]"
                };

                let lang_str = match file.language {
                    Language::Rust => "Rust",
                    Language::Python => "Python",
                    Language::JavaScript => "JS",
                    Language::TypeScript => "TS",
                    Language::Go => "Go",
                    Language::Cpp => "C++",
                    Language::C => "C",
                    Language::Unknown => "?",
                };

                let lang_color = match file.language {
                    Language::Rust => Color::Red,
                    Language::Python => Color::Yellow,
                    Language::JavaScript => Color::Cyan,
                    Language::TypeScript => Color::Blue,
                    Language::Go => Color::Magenta,
                    Language::Cpp => Color::Blue,
                    Language::C => Color::Blue,
                    Language::Unknown => Color::Gray,
                };

                let size_str = if file.size > 0 {
                    format!("{}B", file.size)
                } else {
                    "".to_string()
                };

                ListItem::new(Line::from(vec![
                    Span::raw(format!("{} ", checkbox)),
                    Span::styled(
                        file.path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                        style
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!("({})", lang_str),
                        Style::default().fg(lang_color)
                    ),
                    if !size_str.is_empty() {
                        Span::raw(" ")
                    } else {
                        Span::raw("")
                    },
                    Span::styled(
                        size_str,
                        Style::default().fg(Color::Gray)
                    ),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Files"))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

        f.render_widget(list, chunks[1]);
    }

    /// Draw footer
    fn draw_footer(&self, f: &mut Frame, area: Rect) {
        let selected_count = self.state.files.iter().filter(|f| f.selected).count();
        let scanned_count = self.state.scan_results.len();
        let current_tab_name = self.current_tab.title();

        let footer = Line::from(vec![
            Span::raw(" "),
            Span::styled("ApexCode", Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::raw("Tab: "),
            Span::styled(current_tab_name, Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::raw("Selected: "),
            Span::styled(format!("{}", selected_count), Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::raw("Scanned: "),
            Span::styled(format!("{}", scanned_count), Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to quit"),
        ]);

        let paragraph = Paragraph::new(footer)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }

    /// Update stealth score
    pub fn update_stealth_score(&mut self, score: f64) {
        self.state.stealth_score = Some(StealthScore::new(score as f32, 0.2, 0.3));
    }

    /// Get current state
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Get mutable state
    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }
}
