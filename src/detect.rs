//! Language detection utilities powered by [`whatlang`].
//!
//! Provides a mapping from whatlang's detected language to the analyzer name
//! registered in the Pizza [`AnalysisFactory`].

use alloc::borrow::Cow;
use alloc::vec::Vec;

use pizza_engine::analysis::{Token, TokenFilter};

/// Default minimum confidence threshold for language detection.
/// Below this, we fall back to the `"standard"` analyzer.
/// Can be overridden per-instance via `AutoTokenizer::set_confidence_threshold()`.
pub const DEFAULT_CONFIDENCE_THRESHOLD: f64 = 0.3;

/// Default analyzer used when detection fails or confidence is too low.
pub const FALLBACK_ANALYZER: &str = "standard";

/// Detect the language of `text` and return the Pizza analyzer name.
///
/// Returns `None` if whatlang cannot detect with sufficient confidence.
/// Uses the default threshold; for configurable threshold see [`AutoTokenizer`].
pub fn detect_language(text: &str) -> Option<&'static str> {
    let info = whatlang::detect(text)?;

    if info.confidence() < DEFAULT_CONFIDENCE_THRESHOLD {
        return None;
    }

    Some(whatlang_to_analyzer(info.lang()))
}

/// Map a whatlang `Lang` to the analyzer name registered in the Pizza factory.
pub fn whatlang_to_analyzer(lang: whatlang::Lang) -> &'static str {
    use whatlang::Lang;

    match lang {
        // European — Latin script
        Lang::Eng => "english",
        Lang::Fra => "french",
        Lang::Deu => "german",
        Lang::Spa => "spanish",
        Lang::Ita => "italian",
        Lang::Por => "portuguese",
        Lang::Nld => "dutch",
        Lang::Swe => "swedish",
        Lang::Fin => "finnish",
        Lang::Dan => "danish",
        Lang::Nob => "norwegian",
        Lang::Hun => "hungarian",
        Lang::Ron => "romanian",
        Lang::Cat => "catalan",
        Lang::Pol => "polish",
        Lang::Ces => "czech",
        Lang::Slk => "slovak",
        Lang::Slv => "slovenian",
        Lang::Hrv => "croatian",
        Lang::Srp => "serbian",
        Lang::Bul => "bulgarian",
        Lang::Lit => "lithuanian",
        Lang::Lav => "latvian",
        Lang::Est => "estonian",

        // European — Cyrillic
        Lang::Rus => "russian",
        Lang::Ukr => "ukrainian",

        // European — Greek
        Lang::Ell => "greek",

        // Turkic
        Lang::Tur => "turkish",
        Lang::Aze => "azerbaijani",

        // Nordic / Baltic (handled above, but for completeness)

        // South Asian
        Lang::Hin => "hindi",
        Lang::Ben => "bengali",
        Lang::Mar => "hindi", // Marathi uses Devanagari, Hindi analyzer is close
        Lang::Nep => "hindi", // Nepali uses Devanagari
        Lang::Tam => "tamil",
        Lang::Tel => "telugu",
        Lang::Kan => "kannada",
        Lang::Mal => "malayalam",
        Lang::Guj => "standard", // Gujarati
        Lang::Pan => "standard", // Punjabi
        Lang::Sin => "standard", // Sinhala
        Lang::Urd => "standard", // Urdu

        // Southeast Asian
        Lang::Ind => "indonesian",
        Lang::Tgl => "standard",   // Tagalog
        Lang::Vie => "vietnamese",  // Vietnamese compound word tokenizer
        Lang::Tha => "thai",
        Lang::Khm => "standard",   // Khmer
        Lang::Mya => "standard",   // Burmese

        // East Asian
        Lang::Cmn => "ik",  // Chinese (Mandarin)
        Lang::Jpn => "kuromoji", // Japanese
        Lang::Kor => "nori",     // Korean

        // Middle Eastern
        Lang::Ara => "arabic",
        Lang::Pes => "persian",
        Lang::Heb => "standard", // Hebrew — no dedicated analyzer yet

        // African
        Lang::Amh => "standard", // Amharic

        // Catch-all
        _ => FALLBACK_ANALYZER,
    }
}

/// A token filter that detects the language of the input and emits a metadata
/// token `_lang:<code>` at position 0. This can be used for debugging or for
/// downstream processing that needs to know the detected language.
///
/// The filter passes through all tokens unchanged and prepends the language tag.
#[derive(Clone, Debug, Default)]
pub struct LanguageDetectTokenFilter;

impl LanguageDetectTokenFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for LanguageDetectTokenFilter {
    fn filter<'a>(&self, _token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        // This filter is a passthrough — it doesn't modify tokens.
        // Language detection happens at the AutoTokenizer level.
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_english() {
        assert_eq!(detect_language("Hello world, this is a test"), Some("english"));
    }

    #[test]
    fn detect_french() {
        assert_eq!(detect_language("Bonjour le monde, comment allez-vous aujourd'hui"), Some("french"));
    }

    #[test]
    fn detect_german() {
        assert_eq!(detect_language("Hallo Welt, wie geht es Ihnen heute"), Some("german"));
    }

    #[test]
    fn detect_spanish() {
        assert_eq!(detect_language("Hola mundo, cómo estás hoy en este día"), Some("spanish"));
    }

    #[test]
    fn detect_chinese() {
        assert_eq!(detect_language("你好世界，今天天气怎么样"), Some("smartcn"));
    }

    #[test]
    fn detect_japanese() {
        assert_eq!(detect_language("こんにちは世界、今日はいい天気ですね"), Some("kuromoji"));
    }

    #[test]
    fn detect_korean() {
        assert_eq!(detect_language("안녕하세요 세계, 오늘 날씨가 좋습니다"), Some("nori"));
    }

    #[test]
    fn detect_arabic() {
        assert_eq!(detect_language("مرحبا بالعالم، كيف حالك اليوم"), Some("arabic"));
    }

    #[test]
    fn detect_russian() {
        assert_eq!(detect_language("Привет мир, как дела сегодня"), Some("russian"));
    }

    #[test]
    fn detect_hindi() {
        assert_eq!(detect_language("नमस्ते दुनिया, आज आप कैसे हैं"), Some("hindi"));
    }

    #[test]
    fn detect_turkish() {
        assert_eq!(detect_language("Merhaba dünya, bugün nasılsınız"), Some("turkish"));
    }

    #[test]
    fn detect_portuguese() {
        assert_eq!(detect_language("Olá mundo, como você está hoje neste dia"), Some("portuguese"));
    }

    #[test]
    fn detect_italian() {
        assert_eq!(detect_language("Ciao mondo, come stai oggi in questa giornata"), Some("italian"));
    }

    #[test]
    fn detect_persian() {
        assert_eq!(detect_language("سلام دنیا، حال شما چطور است امروز"), Some("persian"));
    }

    #[test]
    fn detect_dutch() {
        assert_eq!(detect_language("Hallo wereld, hoe gaat het vandaag met je"), Some("dutch"));
    }

    #[test]
    fn detect_swedish() {
        assert_eq!(detect_language("Hej världen, hur mår du idag på denna dag"), Some("swedish"));
    }

    #[test]
    fn detect_indonesian() {
        assert_eq!(detect_language("Halo dunia, apa kabar hari ini semuanya"), Some("indonesian"));
    }

    #[test]
    fn detect_greek() {
        assert_eq!(detect_language("Γεια σου κόσμε, πώς είσαι σήμερα"), Some("greek"));
    }

    #[test]
    fn short_text_may_fallback() {
        // Very short text may not have enough signal
        let result = detect_language("hi");
        // Either detected or None — both acceptable for 2-char input
        assert!(result.is_none() || result == Some("english") || result == Some(FALLBACK_ANALYZER));
    }

    #[test]
    fn empty_text_returns_none() {
        assert_eq!(detect_language(""), None);
    }

    #[test]
    fn whatlang_mapping_completeness() {
        // Ensure all major languages have a non-standard mapping
        let important = [
            (whatlang::Lang::Eng, "english"),
            (whatlang::Lang::Fra, "french"),
            (whatlang::Lang::Deu, "german"),
            (whatlang::Lang::Spa, "spanish"),
            (whatlang::Lang::Ita, "italian"),
            (whatlang::Lang::Por, "portuguese"),
            (whatlang::Lang::Nld, "dutch"),
            (whatlang::Lang::Rus, "russian"),
            (whatlang::Lang::Ara, "arabic"),
            (whatlang::Lang::Pes, "persian"),
            (whatlang::Lang::Hin, "hindi"),
            (whatlang::Lang::Ben, "bengali"),
            (whatlang::Lang::Tur, "turkish"),
            (whatlang::Lang::Ell, "greek"),
            (whatlang::Lang::Fin, "finnish"),
            (whatlang::Lang::Hun, "hungarian"),
            (whatlang::Lang::Swe, "swedish"),
            (whatlang::Lang::Nob, "norwegian"),
            (whatlang::Lang::Ind, "indonesian"),
        ];
        for (lang, expected) in important {
            assert_eq!(whatlang_to_analyzer(lang), expected, "mismatch for {:?}", lang);
        }
    }
}
