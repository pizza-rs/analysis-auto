//! The [`AutoTokenizer`] detects the input language at tokenization time and
//! delegates to the appropriate language analyzer's full pipeline.
//!
//! Because [`Analyzer`] applies normalizers → tokenizer → token-filters in
//! sequence, and the `auto` analyzer must pick the *entire* language-specific
//! pipeline, we run the matched analyzer's full `analyze_and_return_tokens`
//! inside `tokenize()`. The outer `Analyzer` wrapping `AutoTokenizer` therefore
//! has no normalizers or token-filters of its own.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use hashbrown::HashMap;
use pizza_engine::analysis::{Analyzer, Token, Tokenizer};

use crate::detect::{detect_language, FALLBACK_ANALYZER};

/// A tokenizer that detects the language of the input text and delegates to the
/// matching language analyzer's full analysis pipeline.
///
/// Holds clones of all registered language analyzers captured at registration
/// time. When `tokenize()` is called:
///
/// 1. `whatlang::detect(text)` identifies the language
/// 2. The matching analyzer's full pipeline (normalizers + tokenizer + filters)
///    is applied
/// 3. Resulting tokens are returned
///
/// If detection fails or confidence is below threshold, falls back to the
/// `"standard"` analyzer.
#[derive(Clone)]
pub struct AutoTokenizer {
    /// Map from language/analyzer name → cloned Analyzer pipeline.
    analyzers: HashMap<String, Analyzer>,
    /// The fallback analyzer used when detection fails.
    fallback: Analyzer,
}

impl AutoTokenizer {
    /// Build an `AutoTokenizer` by capturing clones of all registered analyzers
    /// from the factory. Called during `register_all()`.
    pub fn new(analyzers: HashMap<String, Analyzer>, fallback: Analyzer) -> Self {
        Self { analyzers, fallback }
    }

    /// Get the analyzer for the detected (or fallback) language.
    fn select_analyzer(&self, text: &str) -> &Analyzer {
        match detect_language(text) {
            Some(lang_name) => self.analyzers.get(lang_name).unwrap_or(&self.fallback),
            None => &self.fallback,
        }
    }
}

impl Tokenizer for AutoTokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> Vec<Token<'a>> {
        let analyzer = self.select_analyzer(text);

        // We need an owned copy because the selected analyzer's normalizers
        // mutate the string in-place before tokenizing.
        let mut input = String::from(text);
        analyzer.analyze_and_return_tokens(&mut input)
            .into_iter()
            .map(|t| Token {
                // Convert back to owned since the original `input` will be dropped
                term: Cow::Owned(t.term.into_owned()),
                start_offset: t.start_offset,
                end_offset: t.end_offset,
                position: t.position,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pizza_engine::analysis::{Analyzer, Tokenizer as _};
    use pizza_engine::analysis::tokenizers::StandardTokenizer;

    fn make_auto() -> AutoTokenizer {
        // Simple test setup: only "standard" in the map
        let standard = Analyzer::new(
            vec![],
            Box::new(StandardTokenizer::new()),
            vec![],
        );
        let mut map = HashMap::new();
        map.insert("standard".into(), standard.clone());
        map.insert("english".into(), standard.clone());
        AutoTokenizer::new(map, standard)
    }

    #[test]
    fn tokenizes_english_text() {
        let auto = make_auto();
        let tokens = auto.tokenize("Hello world this is a simple test sentence");
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].term, "Hello");
    }

    #[test]
    fn tokenizes_empty_text() {
        let auto = make_auto();
        let tokens = auto.tokenize("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn fallback_for_short_text() {
        let auto = make_auto();
        // Very short text — should still produce tokens via fallback
        let tokens = auto.tokenize("hi");
        assert!(!tokens.is_empty());
    }
}
