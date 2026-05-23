<div align="center">

# 🌍 pizza-analysis-auto

**Automatic language-detection analyzer for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--auto-blue)](https://github.com/pizza-rs/analysis-auto)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Detects the language of incoming text at analysis time using the
[whatlang](https://crates.io/crates/whatlang) crate, then delegates to the
matching language-specific analyzer already registered in the `AnalysisFactory`.

When detection confidence is too low, falls back to the `standard` analyzer.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| Analyzer | `auto` | Detects language → delegates to the matching analyzer |
| TokenFilter | `language_detect` | Passthrough filter (placeholder for custom pipelines) |

## Supported Languages

The auto analyzer maps 50+ detected languages to Pizza analyzers:

| Language | Analyzer | Language | Analyzer |
|:---------|:---------|:---------|:---------|
| English | `english` | Arabic | `arabic` |
| French | `french` | Persian | `persian` |
| German | `german` | Hindi | `hindi` |
| Spanish | `spanish` | Bengali | `bengali` |
| Italian | `italian` | Indonesian | `indonesian` |
| Portuguese | `portuguese` | Thai | `thai` |
| Dutch | `dutch` | Chinese | `cjk` |
| Russian | `russian` | Japanese | `cjk` |
| Greek | `greek` | Korean | `cjk` |
| Norwegian | `norwegian` | Turkish | `turkish` |
| Swedish | `swedish` | Hungarian | `hungarian` |
| Finnish | `finnish` | Polish | `polish` |

Languages without a dedicated analyzer (Tamil, Hebrew, Vietnamese, etc.) fall
back to `standard`.

## How It Works

1. **Detection** — whatlang analyzes the input text and returns a language + confidence score
2. **Threshold** — if confidence ≥ 0.3, the detected language is used; otherwise falls back to `standard`
3. **Delegation** — the `AutoTokenizer` runs the full analysis pipeline (normalizers → tokenizer → token filters) of the matched language analyzer

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();

// Register language analyzers first
pizza_analysis_english::register_all(&mut factory);
pizza_analysis_french::register_all(&mut factory);
pizza_analysis_cjk::register_all(&mut factory);

// Register auto last — captures all analyzers above
pizza_analysis_auto::register_all(&mut factory);

let analyzer = factory.get_analyzer("auto").unwrap();
// "Bonjour le monde"  → detected: French  → uses "french" analyzer
// "こんにちは世界"      → detected: Japanese → uses "cjk" analyzer
// "Hello world"       → detected: English → uses "english" analyzer
```

## Installation

```toml
[dependencies]
pizza-analysis-auto = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["auto"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
