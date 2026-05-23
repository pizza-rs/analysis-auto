//! Registration of the `auto` analyzer into the Pizza [`AnalysisFactory`].

use alloc::string::String;

use hashbrown::HashMap;
use pizza_engine::analysis::{Analyzer, AnalysisFactory};

use crate::auto_analyzer::AutoTokenizer;
use crate::detect::LanguageDetectTokenFilter;

/// All language analyzer names that the `auto` analyzer can delegate to.
///
/// These must match the names registered by the language-specific analysis
/// crates. When building the `AutoTokenizer`, we capture clones of whichever
/// of these are already registered in the factory.
const LANGUAGE_ANALYZER_NAMES: &[&str] = &[
    // Foundation
    "standard",
    "whitespace",
    // European — Latin
    "english",
    "french",
    "german",
    "spanish",
    "italian",
    "portuguese",
    "dutch",
    "swedish",
    "finnish",
    "danish",
    "norwegian",
    "hungarian",
    "romanian",
    "catalan",
    "polish",
    "czech",
    "slovak",
    "slovenian",
    "croatian",
    "serbian",
    "bulgarian",
    "lithuanian",
    "latvian",
    "estonian",
    // European — Cyrillic
    "russian",
    "ukrainian",
    // European — Greek
    "greek",
    // Turkic
    "turkish",
    "azerbaijani",
    // South Asian
    "hindi",
    "bengali",
    // Southeast Asian
    "indonesian",
    "thai",
    // East Asian
    "cjk",
    // Middle Eastern
    "arabic",
    "persian",
    // South American
    "brazilian",
];

/// Register the `auto` analyzer and `language_detect` token filter.
///
/// **Must be called after** all language-specific analyzers have been
/// registered, so that `AutoTokenizer` can capture their pipelines.
pub fn register_all(factory: &mut AnalysisFactory) {
    // Capture clones of all available language analyzers
    let mut analyzer_map: HashMap<String, Analyzer> = HashMap::new();
    for &name in LANGUAGE_ANALYZER_NAMES {
        if let Some(analyzer) = factory.get_analyzer(name) {
            analyzer_map.insert(String::from(name), analyzer.clone());
        }
    }

    // Build fallback — use "standard" if available, otherwise create a basic one
    let fallback = factory
        .get_analyzer("standard")
        .cloned()
        .unwrap_or_else(|| {
            Analyzer::new(
                vec![],
                Box::new(pizza_engine::analysis::tokenizers::StandardTokenizer::new()),
                vec![],
            )
        });

    let auto_tokenizer = AutoTokenizer::new(analyzer_map, fallback);

    // Register the `auto` analyzer: AutoTokenizer does all the work internally,
    // so the wrapping Analyzer has no normalizers or token filters.
    let auto_analyzer = Analyzer::new(
        vec![],                    // no normalizers (handled by delegated analyzer)
        Box::new(auto_tokenizer),  // language-detecting tokenizer
        vec![],                    // no token filters (handled by delegated analyzer)
    );
    factory.register_analyzer("auto", auto_analyzer);

    // Register the standalone language_detect token filter for use in custom pipelines
    factory.register_token_filter("language_detect", Box::new(LanguageDetectTokenFilter::new()));
}
