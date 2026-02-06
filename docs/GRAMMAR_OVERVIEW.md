# Yiddish Grammar System Overview

**Project:** Judiw Terminal - Wordform View
**Reference:** Uriel Weinreich, "College Yiddish" (YIVO Institute)
**Date:** 2025-02-06
**Status:** Implemented in `src/wordform_view.rs`

---

## 1. Grammar Classification System

### 1.1 Verb Classes

| Class | Pattern | Example | Past Participle |
|-------|---------|---------|-----------------|
| Weak | ge--t suffix | machn → gemakht | ge- + stem + -t |
| Strong | Vowel shift | zogn → gezoogn | ge- + vowel change |
| Mixed | Vowel shift + -t | | vowel change + -t |
| Hebrew | -n in present, special patterns | | Varies by origin |
| Modal | Special conjugation | knnen, megn | No ge- prefix |
| Auxiliary | hobn, zayn | hobn, gegebn | Special treatment |

### 1.2 Verb Person & Number

| Person | Yiddish | Romanization | Abbrev |
|--------|---------|--------------|-------|
| 1st singular | איך | ikh | 1sg |
| 2nd singular | דו | du | 2sg |
| 3rd singular (m) | ער | er | 3sg.m |
| 3rd singular (f) | זי | zi | 3sg.f |
| 1st plural | מיר | mir | 1pl |
| 2nd plural | איר | ir | 2pl |
| 3rd plural | זיי | zey | 3pl |

### 1.3 Noun Gender & Articles

| Gender | Article (Definite) | Yiddish |
|--------|-------------------|---------|
| Masculine | der | דער |
| Feminine | di | די |
| Neuter | dos | דאָס |
| Plural (all) | di | די |

### 1.4 Plural Formation Patterns

| Pattern | Description | Example |
|---------|-------------|---------|
| None | No change | khosn → khosn |
| S | Add -s | yingl → yingls |
| ES | Add -es | boy → boyes |
| VowelChange | Vowel shift only | tod → teder |
| ErWithVowelChange | -er + vowel shift | man → mener |
| Er | Add -er | kind → kinder |
| N | Add -n | (varies) |
| HebrewIm | Hebrew -im (masc) | |
| HebrewOs | Hebrew -os (fem) | |
| Mixed | Irregular | |

---

## 2. Implementation Details

### 2.1 Data Structures

```rust
// Verb classification
pub enum VerbClass {
    Weak,       // ge--t
    Strong,     // vowel shift
    Mixed,      // vowel + -t
    Hebrew,     // Hebrew origin
    Irregular,  // doesn't fit patterns
    Modal,      // knnen, megn, etc.
    Auxiliary,  // hobn, zayn
}

// Verb conjugation person
pub enum VerbPerson {
    FirstSingular,
    SecondSingular,
    ThirdSingularMasculine,
    ThirdSingularFeminine,
    FirstPlural,
    SecondPlural,
    ThirdPlural,
    ImperativeSingular,
    ImperativePlural,
}
```

### 2.2 Present Tense Conjugation Pattern

For **weak verbs** like "machn" (to make/do):

| Person | Yiddish | Formation |
|--------|---------|-----------|
| 1sg | מאַך | stem (no ending) |
| 2sg | מאַכסט | stem + -st |
| 3sg.m/f | מאַכט | stem + -t |
| 1pl | מאַכן | stem + -n |
| 2pl | מאַכט | stem + -t |
| 3pl | מאַכן | stem + -n |

### 2.3 Past Participle Formation

Weak verbs use: **ge-** + stem + **-t**

- machn → gemakht
- zogn → gezoogn (strong, vowel change)

### 2.4 Imperative Forms

- Singular: stem (no ending)
- Plural: stem + -t

For "machn":
- Singular: מאַך (makh)
- Plural: מאַכט (makht)

---

## 3. Display Format

### 3.1 Verb Paradigm Output

```
═══════════════════════════════════════════════════════════════
VERB CONJUGATION (Weinreich)
═══════════════════════════════════════════════════════════════

Class: Weak (ge--t)
Infinitive: מאַכן (machn)

PRESENT TENSE
───────────────────────────────────────────────────────────────
Person                | Yiddish              | Romanized
──────────────────────────────────────────────────────────────
איך (ikh)             | מאַך                 | makh
דו (du)               | מאַכסט               | makhest
ער (er)               | מאַכט               | makht
...

PAST PARTICIPLE
───────────────────────────────────────────────────────────────
געמאַכט (gemakht)

IMPERATIVE
───────────────────────────────────────────────────────────────
Singular: מאַך (makh)
Plural:   מאַכט (makht)
```

### 3.2 Noun Paradigm Output

```
═══════════════════════════════════════════════════════════════
NOUN DECLENSION (Weinreich)
═══════════════════════════════════════════════════════════════

Gender:   Masculine (דער)
Article:  דער (der)

NUMBER
───────────────────────────────────────────────────────────────
Singular: יונג (yung)
Plural:   יונגן (yungn) [Add -n]
```

---

## 4. Design Decisions

### 4.1 Verb Class Detection

Current implementation uses simple string matching:

```rust
fn classify_verb(lexeme: &Lexeme) -> VerbClass {
    // Check for auxiliary verbs
    if roman == "hobn" || roman == "zayn" → Auxiliary

    // Check for modal verbs
    if ["knen", "megn", "muzn", "voln", "zoln", "darfn"].contains(&roman) → Modal

    // Check for Hebrew origin (from origin field)
    if origin.contains("hebrew") || origin.contains("aramaic") → Hebrew

    // Default to weak (most Yiddish verbs)
    → Weak
}
```

**Limitation:** Full morphological analysis not yet implemented. Currently hardcoded for "machn" as example.

### 4.2 Missing Features

The following features are planned but not yet implemented:

1. **Proper verb stem extraction** - Currently uses simple suffix stripping
2. **Vowel shift detection** - Strong verb conjugation requires this
3. **Morphological feature database** - Would enable automatic pattern detection
4. **Adjective declension** - Tables shown, but not populated with real data
5. **Noun plural prediction** - Pattern detection needs development

---

## 5. Code Locations

- `src/models/shared.rs` - VerbClass, VerbPerson enums
- `src/models/morph_features.rs` - Morphological feature types
- `src/wordform_view.rs` - Display logic and formatting
- `src/morphology.rs` - Morphological analyzer (basic)

---

## 6. Testing Status

- ✅ Unit tests pass (26 tests)
- ✅ Integration tests pass (10 tests)
- ✅ Property-based tests pass (10 tests)
- ⚠️  Verb conjugation only works for "machn" example
- ⚠️  General verb conjugation not yet fully implemented

---

## 7. References

1. **Primary:** Uriel Weinreich, "College Yiddish", YIVO Institute
2. **Secondary:** Yiddish grammar resources (would be linked in external docs)

---

## 8. Next Session Priorities

1. **Implement proper verb stem extraction** - Use phonological rules to derive stem from infinitive
2. **Add strong verb detection** - Vowel shift patterns for common strong verbs
3. **Expand conjugation database** - Store known conjugations for lookup
4. **Implement noun plural prediction** - Pattern-based plural generation
5. **Add adjective declension** - Full attributive declension table

---

*End of Overview*
