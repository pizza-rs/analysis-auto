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

### European — Latin Script

| Language | Code | Analyzer | Language | Code | Analyzer |
|:---------|:-----|:---------|:---------|:-----|:---------|
| English | `eng` | `english` | Norwegian | `nor` | `norwegian` |
| French | `fra` | `french` | Swedish | `swe` | `swedish` |
| German | `deu` | `german` | Finnish | `fin` | `finnish` |
| Spanish | `spa` | `spanish` | Hungarian | `hun` | `hungarian` |
| Italian | `ita` | `italian` | Romanian | `ron` | `romanian` |
| Portuguese | `por` | `portuguese` | Catalan | `cat` | `catalan` |
| Dutch | `nld` | `dutch` | Polish | `pol` | `polish` |
| Danish | `dan` | `danish` | Czech | `ces` | `czech` |
| Slovak | `slk` | `slovak` | Slovenian | `slv` | `slovenian` |
| Croatian | `hrv` | `croatian` | Serbian | `srp` | `serbian` |
| Bulgarian | `bul` | `bulgarian` | Lithuanian | `lit` | `lithuanian` |
| Latvian | `lav` | `latvian` | Estonian | `est` | `estonian` |

### European — Cyrillic / Greek

| Language | Code | Analyzer |
|:---------|:-----|:---------|
| Russian | `rus` | `russian` |
| Ukrainian | `ukr` | `ukrainian` |
| Greek | `ell` | `greek` |

### Turkic

| Language | Code | Analyzer |
|:---------|:-----|:---------|
| Turkish | `tur` | `turkish` |
| Azerbaijani | `aze` | `azerbaijani` |

### South Asian (Indic)

| Language | Code | Analyzer | Notes |
|:---------|:-----|:---------|:------|
| Hindi | `hin` | `hindi` | |
| Bengali | `ben` | `bengali` | |
| Tamil | `tam` | `tamil` | Indic norm + Tamil stem |
| Telugu | `tel` | `telugu` | Indic norm + Telugu stem |
| Kannada | `kan` | `kannada` | Indic norm + Kannada stem |
| Malayalam | `mal` | `malayalam` | Indic norm + chillu normalization |
| Marathi | `mar` | `hindi` | Devanagari script, close to Hindi |
| Nepali | `nep` | `hindi` | Devanagari script, close to Hindi |

### Southeast Asian

| Language | Code | Analyzer |
|:---------|:-----|:---------|
| Indonesian | `ind` | `indonesian` |
| Malay | `msa` | `indonesian` |
| Vietnamese | `vie` | `vietnamese` |
| Thai | `tha` | `thai` |

### East Asian

| Language | Code | Analyzer |
|:---------|:-----|:---------|
| Chinese | `cmn` | `ik` |
| Japanese | `jpn` | `kuromoji` |
| Korean | `kor` | `nori` |

### Middle Eastern

| Language | Code | Analyzer |
|:---------|:-----|:---------|
| Arabic | `ara` | `arabic` |
| Persian | `fas` | `persian` |

> Languages not listed above (Gujarati, Punjabi, Hebrew, Khmer, etc.) fall
> back to `standard`. You can override any mapping — see below.

## Language Overrides

Override the default analyzer for any detected language using its 3-letter code:

```rust
// Use jieba for Chinese instead of the default ik
auto_tokenizer.set_override("cmn", "jieba");
// Use cjk bigram for Japanese instead of kuromoji
auto_tokenizer.set_override("jpn", "cjk");
```

## Confidence Threshold

The default threshold is **0.3**. Adjust it to be stricter or more permissive:

```rust
auto_tokenizer.set_confidence_threshold(0.5); // stricter — require higher confidence
auto_tokenizer.set_confidence_threshold(0.1); // looser — accept weaker detections
```

## How It Works

1. **Detection** — whatlang analyzes the input text and returns a language + confidence score
2. **Threshold** — if confidence ≥ threshold (default 0.3, configurable), the detected language is used; otherwise falls back to `standard`
3. **Override check** — if the user configured an override for the detected language, use that analyzer instead
4. **Delegation** — the `AutoTokenizer` runs the full analysis pipeline (normalizers → tokenizer → token filters) of the matched language analyzer

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();

// Register language analyzers first
pizza_analysis_english::register_all(&mut factory);
pizza_analysis_french::register_all(&mut factory);
pizza_analysis_ik::register_all(&mut factory);
pizza_analysis_kuromoji::register_all(&mut factory);

// Register auto last — captures all analyzers above
pizza_analysis_auto::register_all(&mut factory);

let analyzer = factory.get_analyzer("auto").unwrap();
// "Bonjour le monde"  → detected: French    → uses "french" analyzer
// "こんにちは世界"      → detected: Japanese  → uses "kuromoji" analyzer
// "Hello world"       → detected: English   → uses "english" analyzer
// "北京欢迎你"         → detected: Chinese   → uses "ik" analyzer
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
