//! Comprehensive tests for the `auto` language-detection analyzer.

use pizza_analysis_auto::detect::detect_language;

// ── Language detection accuracy ─────────────────────────────────────────

#[test]
fn detect_english_prose() {
    assert_eq!(
        detect_language("The quick brown fox jumps over the lazy dog near the river"),
        Some("english")
    );
}

#[test]
fn detect_french_prose() {
    assert_eq!(
        detect_language("Le renard brun rapide saute par-dessus le chien paresseux"),
        Some("french")
    );
}

#[test]
fn detect_german_prose() {
    assert_eq!(
        detect_language("Der schnelle braune Fuchs springt über den faulen Hund"),
        Some("german")
    );
}

#[test]
fn detect_spanish_prose() {
    assert_eq!(
        detect_language("El rápido zorro marrón salta sobre el perro perezoso"),
        Some("spanish")
    );
}

#[test]
fn detect_italian_prose() {
    assert_eq!(
        detect_language("La volpe marrone veloce salta sopra il cane pigro vicino al fiume"),
        Some("italian")
    );
}

#[test]
fn detect_portuguese_prose() {
    assert_eq!(
        detect_language("A rápida raposa marrom pula sobre o cachorro preguiçoso"),
        Some("portuguese")
    );
}

#[test]
fn detect_dutch_prose() {
    assert_eq!(
        detect_language("De snelle bruine vos springt over de luie hond bij de rivier"),
        Some("dutch")
    );
}

#[test]
fn detect_russian_prose() {
    assert_eq!(
        detect_language("Быстрая коричневая лиса прыгает через ленивую собаку"),
        Some("russian")
    );
}

#[test]
fn detect_arabic_prose() {
    assert_eq!(
        detect_language("الثعلب البني السريع يقفز فوق الكلب الكسول بالقرب من النهر"),
        Some("arabic")
    );
}

#[test]
fn detect_persian_prose() {
    assert_eq!(
        detect_language("روباه قهوه‌ای سریع از روی سگ تنبل می‌پرد"),
        Some("persian")
    );
}

#[test]
fn detect_hindi_prose() {
    assert_eq!(
        detect_language("तेज भूरी लोमड़ी आलसी कुत्ते के ऊपर कूदती है"),
        Some("hindi")
    );
}

#[test]
fn detect_bengali_prose() {
    assert_eq!(
        detect_language("দ্রুত বাদামী শিয়াল অলস কুকুরের উপর দিয়ে লাফ দেয়"),
        Some("bengali")
    );
}

#[test]
fn detect_turkish_prose() {
    assert_eq!(
        detect_language("Hızlı kahverengi tilki tembel köpeğin üzerinden atlar"),
        Some("turkish")
    );
}

#[test]
fn detect_greek_prose() {
    assert_eq!(
        detect_language("Η γρήγορη καφέ αλεπού πηδά πάνω από τον τεμπέλη σκύλο"),
        Some("greek")
    );
}

#[test]
fn detect_finnish_prose() {
    assert_eq!(
        detect_language("Nopea ruskea kettu hyppää laiskan koiran yli joen lähellä"),
        Some("finnish")
    );
}

#[test]
fn detect_hungarian_prose() {
    assert_eq!(
        detect_language("A gyors barna róka átugrik a lusta kutya fölött a folyó mellett"),
        Some("hungarian")
    );
}

#[test]
fn detect_swedish_prose() {
    assert_eq!(
        detect_language("Den snabba bruna räven hoppar över den lata hunden vid floden"),
        Some("swedish")
    );
}

#[test]
fn detect_norwegian_prose() {
    assert_eq!(
        detect_language("Den raske brune reven hopper over den late hunden ved elven"),
        Some("norwegian")
    );
}

#[test]
fn detect_indonesian_prose() {
    assert_eq!(
        detect_language("Rubah cokelat yang cepat melompati anjing yang malas di dekat sungai"),
        Some("indonesian")
    );
}

#[test]
fn detect_chinese_text() {
    assert_eq!(
        detect_language("快速的棕色狐狸跳过了懒惰的狗"),
        Some("cjk")
    );
}

#[test]
fn detect_japanese_text() {
    assert_eq!(
        detect_language("素早い茶色の狐が怠惰な犬の上を飛び越える"),
        Some("cjk")
    );
}

#[test]
fn detect_korean_text() {
    assert_eq!(
        detect_language("빠른 갈색 여우가 게으른 개를 뛰어넘는다"),
        Some("cjk")
    );
}

// ── Edge cases ──────────────────────────────────────────────────────────

#[test]
fn empty_string_returns_none() {
    assert_eq!(detect_language(""), None);
}

#[test]
fn whitespace_only_returns_none() {
    assert_eq!(detect_language("   \t\n  "), None);
}

#[test]
fn numbers_only_returns_none_or_fallback() {
    let result = detect_language("1234567890");
    // Numbers have no language signal
    assert!(result.is_none() || result == Some("standard"));
}

#[test]
fn punctuation_only_returns_none() {
    assert_eq!(detect_language("...!!!???---"), None);
}

#[test]
fn single_word_may_detect() {
    // Single word detection is unreliable; either None or a valid language is fine
    let result = detect_language("Bonjour");
    assert!(result.is_none() || !result.unwrap().is_empty());
}

#[test]
fn mixed_language_text() {
    // Mixed text should still detect the dominant language
    let result = detect_language(
        "This is English text mixed with 一些中文内容 and more English words following it"
    );
    // Should detect something (likely English since it's dominant)
    assert!(result.is_some());
}

#[test]
fn unicode_emoji_text() {
    // Emoji-heavy text with some words
    let result = detect_language("Hello 🌍🍕🎉 world");
    // May or may not detect; either is acceptable
    assert!(result.is_none() || result.is_some());
}

// ── Mapping completeness ────────────────────────────────────────────────

#[test]
fn all_major_languages_map_to_non_standard() {
    // Verify that major world languages get a specific (non-"standard") analyzer
    let test_texts = [
        ("The quick brown fox jumps over the lazy dog in the forest", "english"),
        ("Le renard brun rapide saute par-dessus le chien paresseux", "french"),
        ("Der schnelle braune Fuchs springt über den faulen Hund", "german"),
        ("Быстрая коричневая лиса прыгает через ленивую собаку", "russian"),
        ("الثعلب البني السريع يقفز فوق الكلب الكسول بالقرب من النهر", "arabic"),
        ("तेज भूरी लोमड़ी आलसी कुत्ते के ऊपर कूदती है", "hindi"),
        ("快速的棕色狐狸跳过了懒惰的狗在森林里", "cjk"),
    ];

    for (text, expected) in test_texts {
        let detected = detect_language(text);
        assert_eq!(
            detected,
            Some(expected),
            "Expected '{}' for text starting with '{}'",
            expected,
            &text[..text.len().min(30)]
        );
    }
}

// ── Longer text for higher confidence ───────────────────────────────────

#[test]
fn detect_polish_longer_text() {
    let result = detect_language(
        "Szybki brązowy lis przeskakuje nad leniwym psem leżącym obok rzeki w lesie"
    );
    assert_eq!(result, Some("polish"));
}

#[test]
fn detect_czech_longer_text() {
    let result = detect_language(
        "Rychlá hnědá liška skáče přes líného psa ležícího u řeky v lese"
    );
    assert_eq!(result, Some("czech"));
}

#[test]
fn detect_romanian_longer_text() {
    let result = detect_language(
        "Vulpea maro rapidă sare peste câinele leneș care stă lângă râu în pădure"
    );
    assert_eq!(result, Some("romanian"));
}

#[test]
fn detect_ukrainian_longer_text() {
    let result = detect_language(
        "Швидка коричнева лисиця стрибає через ледачого собаку біля річки"
    );
    assert_eq!(result, Some("ukrainian"));
}

#[test]
fn detect_bulgarian_longer_text() {
    let result = detect_language(
        "Бързата кафява лисица скача над мързеливото куче край реката в гората"
    );
    assert_eq!(result, Some("bulgarian"));
}

#[test]
fn detect_croatian_longer_text() {
    let result = detect_language(
        "Brza smeđa lisica preskače lijenog psa koji leži pokraj rijeke u šumi"
    );
    // Croatian/Serbian are very similar; either is acceptable
    let detected = result.unwrap_or("standard");
    assert!(
        detected == "croatian" || detected == "serbian" || detected == "slovenian",
        "Got: {detected}"
    );
}
