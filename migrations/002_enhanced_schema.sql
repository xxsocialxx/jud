-- ============================================================================
-- JUDIW TERMINAL: ENHANCED SCHEMA FOR WEINREICH-GRADE LEXICOGRAPHY
-- Version: 2.0
-- Philosophy: Lexeme-Wordform-Sense Trinity with exhaustive metadata
-- ============================================================================

-- ============================================================================
-- SECTION 1: NEW ENUMS FOR RICH METADATA
-- ============================================================================

-- Register (Weinreich-style classification)
CREATE TYPE register_enum AS ENUM (
    'literary',      -- Literary texts
    'colloquial',    -- Everyday speech
    'neutral',       -- Neutral register
    'archaic',       -- Archaic/literary
    'slang',         -- Slang
    'technical'      -- Technical/specialized
);

-- Usage Category (community specificity)
CREATE TYPE usage_category_enum AS ENUM (
    'general',
    'non_hasidic',
    'hasidic_specific',
    'neutral'
);

-- Part of Speech (expanded)
CREATE TYPE pos_enum AS ENUM (
    'noun', 'verb', 'adjective', 'adverb',
    'pronoun', 'preposition', 'conjunction', 'particle',
    'interjection', 'numeral', 'determiner', 'auxiliary'
);

-- Connotation
CREATE TYPE connotation_enum AS ENUM (
    'neutral', 'positive', 'negative', 'euphemistic', 'pejorative', 'ironic'
);

-- Derivation Type
CREATE TYPE derivation_type_enum AS ENUM (
    'diminutive', 'augmentative', 'nominalizer', 'verbalizer',
    'adjectival', 'frequentative', 'privative', 'none'
);

-- Morphological Feature Values
CREATE TYPE gender_enum AS ENUM ('masculine', 'feminine', 'neuter');
CREATE TYPE number_enum AS ENUM ('singular', 'plural', 'dual');
CREATE TYPE tense_enum AS ENUM ('present', 'past', 'future', 'infinitive', 'imperative', 'participle');
CREATE TYPE person_enum AS ENUM ('first', 'second', 'third');
CREATE TYPE case_enum AS ENUM ('nominative', 'accusative', 'dative', 'genitive', 'locative');
CREATE TYPE aspect_enum AS ENUM ('perfective', 'imperfective', 'neutral');
CREATE TYPE definiteness_enum AS ENUM ('definite', 'indefinite');

-- Semantic Relationship Types
CREATE TYPE semantic_relationship_enum AS ENUM (
    'synonym', 'antonym', 'hypernym', 'hyponym',
    'meronym', 'holonym', 'entailment', 'causation'
);

-- ============================================================================
-- SECTION 2: LEXEME ENHANCEMENTS
-- ============================================================================

-- Add Weinreich-style metadata to existing lexeme table
ALTER TABLE linguayi_lexeme
    ADD COLUMN IF NOT EXISTS register register_enum DEFAULT 'neutral',
    ADD COLUMN IF NOT EXISTS usage_category usage_category_enum DEFAULT 'neutral',
    ADD COLUMN IF NOT EXISTS usage_frequency_score REAL,
    ADD COLUMN IF NOT EXISTS etymology_source_language TEXT,
    ADD COLUMN IF NOT EXISTS etymology_source_word TEXT,
    ADD COLUMN IF NOT EXISTS etymology_borrowing_period TEXT,
    ADD COLUMN IF NOT EXISTS etymology_semantic_shift TEXT,
    ADD COLUMN IF NOT EXISTS is_borrowed BOOLEAN DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS notes TEXT,
    ADD COLUMN IF NOT EXISTS cross_references JSONB DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS semantic_tags TEXT[] DEFAULT '{}';

-- Index for register-based queries
CREATE INDEX IF NOT EXISTS idx_lexeme_register ON linguayi_lexeme(register);
CREATE INDEX IF NOT EXISTS idx_lexeme_usage_category ON linguayi_lexeme(usage_category);
CREATE INDEX IF NOT EXISTS idx_lexeme_frequency ON linguayi_lexeme(usage_frequency_score DESC);
CREATE INDEX IF NOT EXISTS idx_lexeme_semantic_tags ON linguayi_lexeme USING GIN(semantic_tags);

-- ============================================================================
-- SECTION 3: MORPHOLOGICAL FEATURES (NEW TABLE)
-- ============================================================================

CREATE TABLE IF NOT EXISTS linguayi_morph_features (
    id BIGSERIAL PRIMARY KEY,
    -- Number & Gender
    number number_enum,
    gender gender_enum,
    definiteness definiteness_enum,

    -- Verbal features
    tense tense_enum,
    aspect aspect_enum,
    person person_enum,

    -- Case (for nouns/pronouns)
    case case_enum,

    -- Other features
    is_reflexive BOOLEAN DEFAULT FALSE,
    is_passive BOOLEAN DEFAULT FALSE,
    is_negated BOOLEAN DEFAULT FALSE,

    -- Derivational info
    derivation_type derivation_type_enum DEFAULT 'none',

    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- SECTION 4: WORDFORM ENHANCEMENTS
-- ============================================================================

-- Link wordforms to morphological features
ALTER TABLE linguayi_wordform
    ADD COLUMN IF NOT EXISTS morph_features_id BIGINT REFERENCES linguayi_morph_features(id),
    ADD COLUMN IF NOT EXISTS base_form_id BIGINT REFERENCES linguayi_wordform(id),
    ADD COLUMN IF NOT EXISTS is_derived BOOLEAN DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS pronunciation_variant TEXT,
    ADD COLUMN IF NOT EXISTS dialect_specific TEXT[],
    ADD COLUMN IF NOT EXISTS corpus_frequency INTEGER DEFAULT 0;

-- Index for wordform queries
CREATE INDEX IF NOT EXISTS idx_wordform_morph_features ON linguayi_wordform(morph_features_id);
CREATE INDEX IF NOT EXISTS idx_wordform_base_form ON linguayi_wordform(base_form_id);
CREATE INDEX IF NOT EXISTS idx_wordform_frequency ON linguayi_wordform(corpus_frequency DESC);

-- ============================================================================
-- SECTION 5: SENSE ENHANCEMENTS
-- ============================================================================

ALTER TABLE linguayi_sense
    ADD COLUMN IF NOT EXISTS definition_number INTEGER,
    ADD COLUMN IF NOT EXISTS definition_english TEXT,
    ADD COLUMN IF NOT EXISTS semantic_field TEXT[],
    ADD COLUMN IF NOT EXISTS register_specific register_enum,
    ADD COLUMN IF NOT EXISTS dialect_specific TEXT[],
    ADD COLUMN IF NOT EXISTS domain_specific TEXT[],
    ADD COLUMN IF NOT EXISTS connotation connotation_enum,
    ADD COLUMN IF NOT EXISTS frequency_in_sense REAL,
    ADD COLUMN IF NOT EXISTS is_primary_sense BOOLEAN DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS usage_notes TEXT;

-- Index for sense queries
CREATE INDEX IF NOT EXISTS idx_sense_semantic_field ON linguayi_sense USING GIN(semantic_field);
CREATE INDEX IF NOT EXISTS idx_sense_register ON linguayi_sense(register_specific);
CREATE INDEX IF NOT EXISTS idx_sense_primary ON linguayi_sense(is_primary_sense) WHERE is_primary_sense = TRUE;

-- ============================================================================
-- SECTION 6: USAGE EXAMPLES (NEW TABLE)
-- ============================================================================

CREATE TABLE IF NOT EXISTS linguayi_usage_example (
    id BIGSERIAL PRIMARY KEY,
    sense_id BIGINT NOT NULL REFERENCES linguayi_sense(id) ON DELETE CASCADE,

    -- Examples in both scripts
    example_yiddish TEXT NOT NULL,
    example_romanized TEXT,
    example_english TEXT NOT NULL,

    -- Source attribution
    source_reference TEXT,
    source_type VARCHAR(50),
    year INTEGER,

    -- Metadata
    is_colloquial BOOLEAN DEFAULT FALSE,
    register register_enum DEFAULT 'neutral',
    notes TEXT,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_usage_example_sense ON linguayi_usage_example(sense_id);
CREATE INDEX IF NOT EXISTS idx_usage_example_yiddish ON linguayi_usage_example USING GIN(example_yiddish gin_trgm_ops);

-- ============================================================================
-- SECTION 7: IDIOMS (NEW TABLE)
-- ============================================================================

CREATE TABLE IF NOT EXISTS linguayi_idiom (
    id BIGSERIAL PRIMARY KEY,

    -- Idiom text
    idiom_yiddish TEXT NOT NULL,
    idiom_romanized TEXT,
    literal_translation TEXT,
    idiomatic_meaning TEXT NOT NULL,

    -- Metadata
    register register_enum DEFAULT 'neutral',
    usage_category usage_category_enum DEFAULT 'neutral',

    -- Relationships
    participating_senses BIGINT[] REFERENCES linguayi_sense(id),
    participating_wordforms BIGINT[] REFERENCES linguayi_wordform(id),

    -- Examples
    example_yiddish TEXT,
    example_english TEXT,

    -- Source
    source_reference TEXT,
    notes TEXT,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_idiom_yiddish ON linguayi_idiom USING GIN(idiom_yiddish gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_idiom_senses ON linguayi_idiom USING GIN(participating_senses);

-- ============================================================================
-- SECTION 8: COLLOCATIONS (NEW TABLE)
-- ============================================================================

CREATE TABLE IF NOT EXISTS linguayi_collocation (
    id BIGSERIAL PRIMARY KEY,

    -- Pattern info
    pattern_type VARCHAR(50) NOT NULL,
    -- 'verb_object', 'adj_noun', 'prep_phrase', etc.

    -- Participating forms
    wordform_ids BIGINT[] NOT NULL,

    -- Frequency and examples
    frequency REAL DEFAULT 0.0,
    examples JSONB DEFAULT '[]',

    -- Semantic info
    semantic_field TEXT,
    register register_enum DEFAULT 'neutral',

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_collocation_wordforms ON linguayi_collocation USING GIN(wordform_ids);
CREATE INDEX IF NOT EXISTS idx_collocation_pattern ON linguayi_collocation(pattern_type);

-- ============================================================================
-- SECTION 9: SEMANTIC RELATIONSHIPS (ENHANCED)
-- ============================================================================

ALTER TABLE linguayi_sense_relationship
    ADD COLUMN IF NOT EXISTS relationship_type semantic_relationship_enum DEFAULT 'synonym',
    ADD COLUMN IF NOT EXISTS confidence_score REAL,
    ADD COLUMN IF NOT EXISTS notes TEXT;

CREATE INDEX IF NOT EXISTS idx_sense_rel_type ON linguayi_sense_relationship(relationship_type);
CREATE INDEX IF NOT EXISTS idx_sense_rel_from ON linguayi_sense_relationship(from_sense_id);
CREATE INDEX IF NOT EXISTS idx_sense_rel_to ON linguayi_sense_relationship(to_sense_id);

-- ============================================================================
-- SECTION 10: ETYMOLOGY (NEW TABLE - DETAILED TRACKING)
-- ============================================================================

CREATE TABLE IF NOT EXISTS linguayi_etymology (
    id BIGSERIAL PRIMARY KEY,
    lexeme_id UUID NOT NULL REFERENCES linguayi_lexeme(id) ON DELETE CASCADE,

    -- Source info
    source_language TEXT NOT NULL,
    source_word TEXT,
    source_romanization TEXT,
    source_meaning TEXT,

    -- Borrowing info
    borrowing_period TEXT,
    borrowing_route TEXT[],  -- e.g., ['Hebrew', 'Aramaic', 'Yiddish']
    semantic_shift TEXT,
    phonological_shift TEXT,

    -- Attestation
    earliest_attestation INTEGER,
    attestation_reference TEXT,

    -- Confidence
    confidence_score REAL,
    is_certain BOOLEAN DEFAULT FALSE,
    notes TEXT,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_etymology_lexeme ON linguayi_etymology(lexeme_id);
CREATE INDEX IF NOT EXISTS idx_etymology_language ON linguayi_etymology(source_language);

-- ============================================================================
-- SECTION 11: ATTESTATIONS (CORPUS REFERENCES)
-- ============================================================================

CREATE TABLE IF NOT EXISTS linguayi_attestation (
    id BIGSERIAL PRIMARY KEY,
    wordform_id BIGINT NOT NULL REFERENCES linguayi_wordform(id) ON DELETE CASCADE,

    -- Source info
    source_title TEXT,
    source_author TEXT,
    source_year INTEGER,
    source_type VARCHAR(50),

    -- Location
    page_reference TEXT,
    line_reference TEXT,
    url TEXT,

    -- Context
    context_sentence TEXT,
    context_romanization TEXT,

    -- Metadata
    is_first_attestation BOOLEAN DEFAULT FALSE,
    notes TEXT,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_attestation_wordform ON linguayi_attestation(wordform_id);
CREATE INDEX IF NOT EXISTS idx_attestation_source ON linguayi_attestation(source_type, source_year);

-- ============================================================================
-- SECTION 12: HASIDIC COMMUNITY DATA (NEW TABLE)
-- ============================================================================

CREATE TABLE IF NOT EXISTS linguayi_hasidic_community (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    english_name VARCHAR(100),
    geographic_origin TEXT,
    founding_year INTEGER,

    -- Dialect features
    phonological_features JSONB,
    lexical_specificities TEXT[],

    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Many-to-many relationship: lexemes used in specific Hasidic communities
CREATE TABLE IF NOT EXISTS linguayi_lexeme_hasidic_usage (
    lexeme_id UUID NOT NULL REFERENCES linguayi_lexeme(id) ON DELETE CASCADE,
    community_id BIGINT NOT NULL REFERENCES linguayi_hasidic_community(id) ON DELETE CASCADE,

    -- Usage specifics
    is_specific BOOLEAN DEFAULT FALSE,
    frequency REAL,
    notes TEXT,

    PRIMARY KEY (lexeme_id, community_id)
);

CREATE INDEX IF NOT EXISTS idx_lexeme_hasidic_lexeme ON linguayi_lexeme_hasidic_usage(lexeme_id);
CREATE INDEX IF NOT EXISTS idx_lexeme_hasidic_community ON linguayi_lexeme_hasidic_usage(community_id);

-- ============================================================================
-- SECTION 13: FULL-TEXT SEARCH CONFIGURATION
-- ============================================================================

-- Create text search configuration for Yiddish (using Hebrew rules)
CREATE TEXT SEARCH CONFIGURATION IF NOT EXISTS yiddish (COPY = hebrew);

-- Full-text search indexes
CREATE INDEX IF NOT EXISTS idx_lexeme_fts ON linguayi_lexeme USING GIN(
    to_tsvector('yiddish',
        COALESCE(canonical_hebrew, '') || ' ' ||
        COALESCE(canonical_roman, '') || ' ' ||
        COALESCE(english_definition, '')
    )
);

CREATE INDEX IF NOT EXISTS idx_sense_fts ON linguayi_sense USING GIN(
    to_tsvector('yiddish',
        COALESCE(definition, '') || ' ' ||
        COALESCE(definition_yi, '') || ' ' ||
        COALESCE(definition_english, '')
    )
);

-- ============================================================================
-- SECTION 14: TRIGGERS FOR UPDATED_AT
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply to relevant tables
DROP TRIGGER IF EXISTS update_lexeme_updated_at ON linguayi_lexeme;
CREATE TRIGGER update_lexeme_updated_at
    BEFORE UPDATE ON linguayi_lexeme
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_idiom_updated_at ON linguayi_idiom;
CREATE TRIGGER update_idiom_updated_at
    BEFORE UPDATE ON linguayi_idiom
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- SECTION 15: VIEWS FOR COMMON QUERIES
-- ============================================================================

-- Complete lexeme view with all metadata
CREATE OR REPLACE VIEW v_lexeme_complete AS
SELECT
    l.*,
    COUNT(DISTINCT w.id) AS wordform_count,
    COUNT(DISTINCT s.id) AS sense_count,
    COUNT(DISTINCT e.id) AS etymology_count,
    COUNT(DISTINCT i.id) AS idiom_count
FROM linguayi_lexeme l
LEFT JOIN linguayi_wordform w ON l.id = w.lexeme_id
LEFT JOIN linguayi_sense s ON l.id = s.lexeme_id
LEFT JOIN linguayi_etymology e ON l.id = e.lexeme_id
LEFT JOIN linguayi_idiom i ON l.id = ANY(i.participating_senses)
GROUP BY l.id;

-- Lexeme search view (for fuzzy search)
CREATE OR REPLACE VIEW v_lexeme_search AS
SELECT
    l.id,
    l.canonical_hebrew,
    l.canonical_roman,
    l.part_of_speech::text,
    l.register,
    l.usage_category,
    l.usage_frequency_score,
    l.english_definition,
    STRING_AGG(DISTINCT w.text, ' ') AS all_wordforms,
    STRING_AGG(DISTINCT s.definition, ' | ') AS all_senses
FROM linguayi_lexeme l
LEFT JOIN linguayi_wordform w ON l.id = w.lexeme_id
LEFT JOIN linguayi_sense s ON l.id = s.lexeme_id
GROUP BY l.id;

-- ============================================================================
-- MIGRATION COMPLETE
-- ============================================================================
-- This schema now supports:
-- ✓ Weinreich-grade register/dialect tracking
-- ✓ Complete morphological feature analysis
-- ✓ Sense relationships and semantic fields
-- ✓ Idioms and collocations
-- ✓ Etymology chains
-- ✓ Corpus attestations
-- ✓ Hasidic community specificity
-- ✓ Full-text search in Yiddish
-- ✓ Hierarchical drill-down from lexeme → wordform → sense
-- ============================================================================
