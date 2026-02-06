-- ============================================================================
-- JUDIW TERMINAL: ENHANCED SCHEMA FOR WEINREICH-GRADE LEXICOGRAPHY
-- Version: 2.0 (Fixed)
-- Philosophy: Lexeme-Wordform-Sense Trinity with exhaustive metadata
-- ============================================================================

-- ============================================================================
-- SECTION 1: NEW ENUMS FOR RICH METADATA
-- ============================================================================

-- Register (Weinreich-style classification)
CREATE TYPE IF NOT EXISTS register_enum AS ENUM (
    'literary',      -- Literary texts
    'colloquial',    -- Everyday speech
    'neutral',       -- Neutral register
    'archaic',       -- Archaic/literary
    'slang',         -- Slang
    'technical'      -- Technical/specialized
);

-- Usage Category (community specificity)
CREATE TYPE IF NOT EXISTS usage_category_enum AS ENUM (
    'general',
    'non_hasidic',
    'hasidic_specific',
    'neutral'
);

-- Connotation
CREATE TYPE IF NOT EXISTS connotation_enum AS ENUM (
    'neutral', 'positive', 'negative', 'euphemistic', 'pejorative', 'ironic'
);

-- Derivation Type
CREATE TYPE IF NOT EXISTS derivation_type_enum AS ENUM (
    'diminutive', 'augmentative', 'nominalizer', 'verbalizer',
    'adjectival', 'frequentative', 'privative', 'none'
);

-- Morphological Feature Values
CREATE TYPE IF NOT EXISTS gender_enum AS ENUM ('masculine', 'feminine', 'neuter');
CREATE TYPE IF NOT EXISTS number_enum AS ENUM ('singular', 'plural', 'dual');
CREATE TYPE IF NOT EXISTS tense_enum AS ENUM ('present', 'past', 'future', 'infinitive', 'imperative', 'participle');
CREATE TYPE IF NOT EXISTS person_enum AS ENUM ('first', 'second', 'third');
CREATE TYPE IF NOT EXISTS case_enum AS ENUM ('nominative', 'accusative', 'dative', 'genitive', 'locative');
CREATE TYPE IF NOT EXISTS aspect_enum AS ENUM ('perfective', 'imperfective', 'neutral');
CREATE TYPE IF NOT EXISTS definiteness_enum AS ENUM ('definite', 'indefinite');

-- Semantic Relationship Types
CREATE TYPE IF NOT EXISTS semantic_relationship_enum AS ENUM (
    'synonym', 'antonym', 'hypernym', 'hyponym',
    'meronym', 'holonym', 'entailment', 'causation'
);

-- ============================================================================
-- SECTION 2: LEXEME ENHANCEMENTS
-- ============================================================================

-- Add Weinreich-style metadata to existing lexeme table
DO $$
BEGIN
    -- Check and add columns individually to avoid errors
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_lexeme' AND column_name = 'register'
    ) THEN
        ALTER TABLE linguayi_lexeme
            ADD COLUMN register register_enum DEFAULT 'neutral';
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_lexeme' AND column_name = 'usage_category'
    ) THEN
        ALTER TABLE linguayi_lexeme
            ADD COLUMN usage_category usage_category_enum DEFAULT 'neutral';
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_lexeme' AND column_name = 'usage_frequency_score'
    ) THEN
        ALTER TABLE linguayi_lexeme
            ADD COLUMN usage_frequency_score REAL;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_lexeme' AND column_name = 'etymology_source_language'
    ) THEN
        ALTER TABLE linguayi_lexeme
            ADD COLUMN etymology_source_language TEXT;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_lexeme' AND column_name = 'etymology_source_word'
    ) THEN
        ALTER TABLE linguayi_lexeme
            ADD COLUMN etymology_source_word TEXT;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_lexeme' AND column_name = 'is_borrowed'
    ) THEN
        ALTER TABLE linguayi_lexeme
            ADD COLUMN is_borrowed BOOLEAN DEFAULT FALSE;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_lexeme' AND column_name = 'semantic_tags'
    ) THEN
        ALTER TABLE linguayi_lexeme
            ADD COLUMN semantic_tags TEXT[] DEFAULT '{}';
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_lexeme' AND column_name = 'notes'
    ) THEN
        ALTER TABLE linguayi_lexeme
            ADD COLUMN notes TEXT;
    END IF;
END $$;

-- Index for register-based queries
CREATE INDEX IF NOT EXISTS idx_lexeme_register ON linguayi_lexeme(register);
CREATE INDEX IF NOT EXISTS idx_lexeme_usage_category ON linguayi_lexeme(usage_category);
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
    case_value case_enum,

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

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_wordform' AND column_name = 'morph_features_id'
    ) THEN
        ALTER TABLE linguayi_wordform
            ADD COLUMN morph_features_id BIGINT REFERENCES linguayi_morph_features(id);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_wordform' AND column_name = 'base_form_id'
    ) THEN
        ALTER TABLE linguayi_wordform
            ADD COLUMN base_form_id BIGINT REFERENCES linguayi_wordform(id);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_wordform' AND column_name = 'is_derived'
    ) THEN
        ALTER TABLE linguayi_wordform
            ADD COLUMN is_derived BOOLEAN DEFAULT FALSE;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_wordform' AND column_name = 'corpus_frequency'
    ) THEN
        ALTER TABLE linguayi_wordform
            ADD COLUMN corpus_frequency INTEGER DEFAULT 0;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_wordform' AND column_name = 'dialect_specific'
    ) THEN
        ALTER TABLE linguayi_wordform
            ADD COLUMN dialect_specific TEXT[];
    END IF;
END $$;

-- Index for wordform queries
CREATE INDEX IF NOT EXISTS idx_wordform_morph_features ON linguayi_wordform(morph_features_id);
CREATE INDEX IF NOT EXISTS idx_wordform_base_form ON linguayi_wordform(base_form_id);
CREATE INDEX IF NOT EXISTS idx_wordform_frequency ON linguayi_wordform(corpus_frequency DESC);

-- ============================================================================
-- SECTION 5: SENSE ENHANCEMENTS
-- ============================================================================

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_sense' AND column_name = 'definition_number'
    ) THEN
        ALTER TABLE linguayi_sense
            ADD COLUMN definition_number INTEGER;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_sense' AND column_name = 'definition_english'
    ) THEN
        ALTER TABLE linguayi_sense
            ADD COLUMN definition_english TEXT;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_sense' AND column_name = 'semantic_field'
    ) THEN
        ALTER TABLE linguayi_sense
            ADD COLUMN semantic_field TEXT[];
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_sense' AND column_name = 'connotation'
    ) THEN
        ALTER TABLE linguayi_sense
            ADD COLUMN connotation connotation_enum;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'linguayi_sense' AND column_name = 'is_primary_sense'
    ) THEN
        ALTER TABLE linguayi_sense
            ADD COLUMN is_primary_sense BOOLEAN DEFAULT FALSE;
    END IF;
END $$;

-- Index for sense queries
CREATE INDEX IF NOT EXISTS idx_sense_semantic_field ON linguayi_sense USING GIN(semantic_field);
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

    -- Relationships (store as arrays, no FK constraint)
    participating_senses BIGINT[],
    participating_wordforms BIGINT[],

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
-- SECTION 9: ETYMOLOGY (NEW TABLE - DETAILED TRACKING)
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
    borrowing_route TEXT[],
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
-- SECTION 10: VIEWS FOR COMMON QUERIES
-- ============================================================================

-- Complete lexeme view with all metadata
CREATE OR REPLACE VIEW v_lexeme_complete AS
SELECT
    l.*,
    COUNT(DISTINCT w.id) AS wordform_count,
    COUNT(DISTINCT s.id) AS sense_count,
    COUNT(DISTINCT e.id) AS etymology_count
FROM linguayi_lexeme l
LEFT JOIN linguayi_wordform w ON l.id = w.lexeme_id
LEFT JOIN linguayi_sense s ON l.id = s.lexeme_id
LEFT JOIN linguayi_etymology e ON l.id = e.lexeme_id
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
-- SECTION 11: TRIGGERS FOR UPDATED_AT
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS update_idiom_updated_at ON linguayi_idiom;
CREATE TRIGGER update_idiom_updated_at
    BEFORE UPDATE ON linguayi_idiom
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- MIGRATION COMPLETE
-- ============================================================================
