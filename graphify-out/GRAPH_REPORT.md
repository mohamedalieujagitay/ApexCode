# Graph Report - .  (2026-04-29)

## Corpus Check
- Corpus is ~12,379 words - fits in a single context window. You may not need a graph.

## Summary
- 187 nodes · 356 edges · 16 communities detected
- Extraction: 89% EXTRACTED · 11% INFERRED · 0% AMBIGUOUS · INFERRED: 40 edges (avg confidence: 0.8)
- Token cost: 12,000 input · 1,000 output

## Community Hubs (Navigation)
- [[_COMMUNITY_CLI & Undercover Engine|CLI & Undercover Engine]]
- [[_COMMUNITY_Core Type Definitions|Core Type Definitions]]
- [[_COMMUNITY_Humanizer Engine|Humanizer Engine]]
- [[_COMMUNITY_TUI Dashboard|TUI Dashboard]]
- [[_COMMUNITY_Detection Engine|Detection Engine]]
- [[_COMMUNITY_Jitter Engine|Jitter Engine]]
- [[_COMMUNITY_Humanizer Scripts|Humanizer Scripts]]
- [[_COMMUNITY_Configuration Module|Configuration Module]]
- [[_COMMUNITY_Stealth Check Scripts|Stealth Check Scripts]]
- [[_COMMUNITY_Naming Convention Scripts|Naming Convention Scripts]]
- [[_COMMUNITY_Project Hub|Project Hub]]
- [[_COMMUNITY_Commit Entropy Scripts|Commit Entropy Scripts]]
- [[_COMMUNITY_Project Overview Docs|Project Overview Docs]]
- [[_COMMUNITY_AI Detection Docs|AI Detection Docs]]
- [[_COMMUNITY_Code Humanization Docs|Code Humanization Docs]]
- [[_COMMUNITY_TUI Visuals|TUI Visuals]]

## God Nodes (most connected - your core abstractions)
1. `UndercoverEngine` - 22 edges
2. `ApexHumanizer` - 16 edges
3. `ApexTui` - 14 edges
4. `main()` - 10 edges
5. `cmd_undercover()` - 9 edges
6. `ApexDetector` - 7 edges
7. `cmd_scan()` - 7 edges
8. `cmd_humanize()` - 7 edges
9. `read_code_block()` - 7 edges
10. `ApexJitter` - 6 edges

## Surprising Connections (you probably didn't know these)
- `ApexDetector` --references--> `Core Crate`  [EXTRACTED]
  crates/detector/src/lib.rs → crates/core/src/lib.rs
- `ApexHumanizer` --references--> `Core Crate`  [EXTRACTED]
  crates/humanizer/src/lib.rs → crates/core/src/lib.rs
- `ApexJitter` --references--> `Core Crate`  [EXTRACTED]
  crates/jitter/src/lib.rs → crates/core/src/lib.rs
- `CLI Crate` --calls--> `ApexDetector`  [EXTRACTED]
  crates/cli/src/main.rs → crates/detector/src/lib.rs
- `CLI Crate` --calls--> `ApexHumanizer`  [EXTRACTED]
  crates/cli/src/main.rs → crates/humanizer/src/lib.rs

## Communities

### Community 0 - "CLI & Undercover Engine"
Cohesion: 0.11
Nodes (20): Persona, test_banned_words_detection(), test_commit_message_transformation(), test_docblock_conversion(), test_word_replacements(), UndercoverConfig, UndercoverEngine, Cli (+12 more)

### Community 1 - "Core Type Definitions"
Cohesion: 0.08
Nodes (21): ApiConfig, CodeBlock, CommentStyle, CommitFormat, CommitMessage, DetectionConfig, DetectionResult, Detector (+13 more)

### Community 2 - "Humanizer Engine"
Cohesion: 0.22
Nodes (5): HumanizerConfig, ApexHumanizer, test_entropy_application(), test_humanizer_creation(), test_variable_renaming()

### Community 3 - "TUI Dashboard"
Cohesion: 0.25
Nodes (5): ApexTui, Tab, test_quit(), test_tab_navigation(), test_tui_creation()

### Community 4 - "Detection Engine"
Cohesion: 0.17
Nodes (9): CloudClient, CloudDetector, DetectorConfig, is_emoji(), LocalModel, ApexDetector, RobertaDetector, test_detector_creation() (+1 more)

### Community 5 - "Jitter Engine"
Cohesion: 0.27
Nodes (5): JitterConfig, ApexJitter, test_apply_jitter(), test_jitter_creation(), test_jitter_enabled()

### Community 6 - "Humanizer Scripts"
Cohesion: 0.36
Nodes (7): get_staged_files(), humanize_content(), humanize_file(), main(), Humanize a single file., Get list of staged files., Humanize code content.

### Community 7 - "Configuration Module"
Cohesion: 0.6
Nodes (2): Config, cmd_config()

### Community 8 - "Stealth Check Scripts"
Cohesion: 0.47
Nodes (5): check_file_for_ai_signatures(), get_staged_files(), main(), Get list of staged files., Check a file for AI signatures.

### Community 9 - "Naming Convention Scripts"
Cohesion: 0.47
Nodes (5): check_file_naming(), get_staged_files(), main(), Get list of staged files., Check a file for over-descriptive naming.

### Community 10 - "Project Hub"
Cohesion: 0.6
Nodes (5): CLI Crate, Core Crate, ApexDetector, ApexHumanizer, ApexJitter

### Community 11 - "Commit Entropy Scripts"
Cohesion: 0.67
Nodes (3): check_commit_message(), main(), Check commit message for AI signatures.

### Community 13 - "Project Overview Docs"
Cohesion: 1.0
Nodes (1): ApexCode

### Community 14 - "AI Detection Docs"
Cohesion: 1.0
Nodes (1): AI Detection

### Community 15 - "Code Humanization Docs"
Cohesion: 1.0
Nodes (1): Code Humanization

### Community 16 - "TUI Visuals"
Cohesion: 1.0
Nodes (1): TUI Dashboard

## Knowledge Gaps
- **36 isolated node(s):** `CodeBlock`, `CommitMessage`, `StyleProfile`, `NamingConvention`, `CommentStyle` (+31 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **Thin community `Configuration Module`** (6 nodes): `Config`, `.default()`, `.load()`, `.load_or_default()`, `.save()`, `cmd_config()`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Project Overview Docs`** (1 nodes): `ApexCode`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `AI Detection Docs`** (1 nodes): `AI Detection`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Code Humanization Docs`** (1 nodes): `Code Humanization`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `TUI Visuals`** (1 nodes): `TUI Dashboard`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `cmd_scan()` connect `CLI & Undercover Engine` to `Core Type Definitions`, `Detection Engine`?**
  _High betweenness centrality (0.189) - this node is a cross-community bridge._
- **Why does `cmd_dashboard()` connect `CLI & Undercover Engine` to `TUI Dashboard`?**
  _High betweenness centrality (0.116) - this node is a cross-community bridge._
- **Why does `cmd_humanize()` connect `CLI & Undercover Engine` to `Humanizer Engine`?**
  _High betweenness centrality (0.113) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `cmd_undercover()` (e.g. with `.new()` and `.transform_commit_message()`) actually correct?**
  _`cmd_undercover()` has 5 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CodeBlock`, `CommitMessage`, `StyleProfile` to the rest of the system?**
  _36 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `CLI & Undercover Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.11 - nodes in this community are weakly interconnected._
- **Should `Core Type Definitions` be split into smaller, more focused modules?**
  _Cohesion score 0.08 - nodes in this community are weakly interconnected._