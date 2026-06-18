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

use crate::detect::FALLBACK_ANALYZER;

/// A tokenizer that detects the language of the input text and delegates to the
/// matching language analyzer's full analysis pipeline.
///
/// Holds clones of all registered language analyzers captured at registration
/// time. When `tokenize()` is called:
///
/// 1. `whatlang::detect(text)` identifies the language
/// 2. If an override is configured for that language, the override analyzer is used
/// 3. Otherwise the default mapping's analyzer pipeline is applied
/// 4. Resulting tokens are returned
///
/// If detection fails or confidence is below threshold, falls back to the
/// `"standard"` analyzer.
///
/// ## Language overrides
///
/// Users can override which analyzer is used for a detected language:
///
/// ```text
/// // Override Chinese to use jieba instead of the default smartcn
/// auto_tokenizer.set_override("cmn", "jieba");
/// // Override Japanese to use cjk bigram instead of kuromoji
/// auto_tokenizer.set_override("jpn", "cjk");
/// ```
#[derive(Clone)]
pub struct AutoTokenizer {
    /// Map from language/analyzer name → cloned Analyzer pipeline.
    analyzers: HashMap<String, Analyzer>,
    /// The fallback analyzer used when detection fails.
    fallback: Analyzer,
    /// User-configured overrides: detected language code → target analyzer name.
    /// Keys are whatlang language codes (e.g. "cmn", "jpn", "kor", "eng").
    /// Values are analyzer names registered in the factory.
    overrides: HashMap<String, String>,
    /// Confidence threshold for language detection (0.0–1.0).
    /// Below this value, the fallback analyzer is used.
    confidence_threshold: f64,
}

impl AutoTokenizer {
    /// Build an `AutoTokenizer` by capturing clones of all registered analyzers
    /// from the factory. Called during `register_all()`.
    pub fn new(analyzers: HashMap<String, Analyzer>, fallback: Analyzer) -> Self {
        Self {
            analyzers,
            fallback,
            overrides: HashMap::new(),
            confidence_threshold: crate::detect::DEFAULT_CONFIDENCE_THRESHOLD,
        }
    }

    /// Set the confidence threshold for language detection.
    ///
    /// Values range from 0.0 (accept any detection) to 1.0 (require perfect confidence).
    /// Default is 0.3.
    pub fn set_confidence_threshold(&mut self, threshold: f64) {
        self.confidence_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Set a language override: when the given language is detected, use the
    /// specified analyzer instead of the default mapping.
    ///
    /// `lang_code` is a whatlang 3-letter code (e.g. `"cmn"`, `"jpn"`, `"eng"`).
    /// `analyzer_name` is the name of an analyzer already registered in the factory.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Use jieba for Chinese instead of the default smartcn
    /// auto_tokenizer.set_override("cmn", "jieba");
    /// ```
    pub fn set_override(&mut self, lang_code: &str, analyzer_name: &str) {
        self.overrides
            .insert(String::from(lang_code), String::from(analyzer_name));
    }

    /// Set multiple overrides at once from a map of lang_code → analyzer_name.
    pub fn set_overrides(&mut self, overrides: HashMap<String, String>) {
        self.overrides = overrides;
    }

    /// Get the analyzer for the detected (or fallback) language,
    /// applying any configured overrides.
    fn select_analyzer(&self, text: &str) -> &Analyzer {
        let info = whatlang::detect(text);

        match info {
            Some(info) if info.confidence() >= self.confidence_threshold => {
                let lang = info.lang();
                let lang_code = lang_to_code(lang);

                // Check for user override first
                if let Some(override_name) = self.overrides.get(lang_code) {
                    if let Some(analyzer) = self.analyzers.get(override_name.as_str()) {
                        return analyzer;
                    }
                }

                // Fall back to default mapping
                let default_name = crate::detect::whatlang_to_analyzer(lang);
                self.analyzers.get(default_name).unwrap_or(&self.fallback)
            }
            _ => &self.fallback,
        }
    }
}

/// Convert a whatlang `Lang` to its ISO 639-3 code string.
///
/// These codes are used as keys in the override map, e.g. `"cmn"` for Chinese,
/// `"jpn"` for Japanese, `"eng"` for English.
fn lang_to_code(lang: whatlang::Lang) -> &'static str {
    use whatlang::Lang;
    match lang {
        Lang::Eng => "eng",
        Lang::Fra => "fra",
        Lang::Deu => "deu",
        Lang::Spa => "spa",
        Lang::Ita => "ita",
        Lang::Por => "por",
        Lang::Nld => "nld",
        Lang::Swe => "swe",
        Lang::Fin => "fin",
        Lang::Dan => "dan",
        Lang::Nob => "nor",
        Lang::Hun => "hun",
        Lang::Ron => "ron",
        Lang::Cat => "cat",
        Lang::Pol => "pol",
        Lang::Ces => "ces",
        Lang::Slk => "slk",
        Lang::Slv => "slv",
        Lang::Hrv => "hrv",
        Lang::Srp => "srp",
        Lang::Bul => "bul",
        Lang::Lit => "lit",
        Lang::Lav => "lav",
        Lang::Est => "est",
        Lang::Rus => "rus",
        Lang::Ukr => "ukr",
        Lang::Ell => "ell",
        Lang::Tur => "tur",
        Lang::Aze => "aze",
        Lang::Hin => "hin",
        Lang::Ben => "ben",
        Lang::Mar => "mar",
        Lang::Nep => "nep",
        Lang::Tam => "tam",
        Lang::Tel => "tel",
        Lang::Kan => "kan",
        Lang::Mal => "mal",
        Lang::Guj => "guj",
        Lang::Pan => "pan",
        Lang::Sin => "sin",
        Lang::Urd => "urd",
        Lang::Ind => "ind",
        Lang::Tgl => "tgl",
        Lang::Vie => "vie",
        Lang::Tha => "tha",
        Lang::Khm => "khm",
        Lang::Mya => "mya",
        Lang::Cmn => "cmn",
        Lang::Jpn => "jpn",
        Lang::Kor => "kor",
        Lang::Ara => "ara",
        Lang::Pes => "fas",
        Lang::Heb => "heb",
        Lang::Amh => "amh",
        _ => "und", // undetermined
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
