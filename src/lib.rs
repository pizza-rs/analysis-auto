//! # pizza-analysis-auto
//!
//! Automatic language-detection analyzer for INFINI Pizza.
//!
//! Uses the [`whatlang`](https://crates.io/crates/whatlang) crate to detect the
//! language of incoming text, then delegates to the appropriate language-specific
//! analyzer already registered in the [`AnalysisFactory`].
//!
//! ## Registered components
//!
//! | Name             | Type     | Description                                          |
//! |------------------|----------|------------------------------------------------------|
//! | `auto`           | Analyzer | Detects language → delegates to the matching analyzer |
//! | `language_detect` | TokenFilter | Adds `_lang:<code>` metadata token                |
//!
//! ## Fallback
//!
//! When whatlang cannot confidently detect the language (confidence below the
//! threshold), the analyzer falls back to `"standard"`.
//!
//! ## Example
//!
//! ```text
//! // Input: "Bonjour le monde"  →  detected: French  →  uses "french" analyzer
//! // Input: "こんにちは世界"      →  detected: Japanese →  uses "cjk" analyzer
//! // Input: "Hello world"       →  detected: English →  uses "english" analyzer
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod auto_analyzer;
mod detect;
pub mod register;

pub use auto_analyzer::AutoTokenizer;
pub use detect::{detect_language, LanguageDetectTokenFilter};
pub use register::register_all;
