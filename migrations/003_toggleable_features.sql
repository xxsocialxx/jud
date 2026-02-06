-- ============================================================================
-- TOGGLEABLE FEATURES - Incremental Enhancement Columns
-- ============================================================================
-- These columns allow incrementally adding LLM-generated features per lexeme
-- without requiring massive data imports or complex systems upfront.
--
-- Philosophy: Add toggleable features per lexeme, don't overcommit to big
-- systems without data. Calculate counts via UPDATE queries, test with
-- existing data, then add features incrementally.
-- ============================================================================

-- ----------------------------------------------------------------------------
-- 1. Example Count Toggle
-- Purpose: Track how many usage examples exist per lexeme
-- LLM Hook: Count examples when LLM generates them
-- ----------------------------------------------------------------------------
ALTER TABLE linguayi_lexeme
ADD COLUMN IF NOT EXISTS example_count INTEGER DEFAULT 0;

-- Index for querying lexemes with examples
CREATE INDEX IF NOT EXISTS idx_lexeme_example_count
ON linguayi_lexeme(example_count) WHERE example_count > 0;

COMMENT ON COLUMN linguayi_lexeme.example_count IS
'Number of usage examples available for this lexeme (LLM-generated or corpus)';

-- ----------------------------------------------------------------------------
-- 2. POS Confidence Toggle
-- Purpose: Track LLM POS tagging confidence scores
-- LLM Hook: Store LLM confidence scores when tagging parts of speech
-- ----------------------------------------------------------------------------
ALTER TABLE linguayi_lexeme
ADD COLUMN IF NOT EXISTS pos_confidence REAL DEFAULT 1.0;

-- Index for low-confidence entries (needs review)
CREATE INDEX IF NOT EXISTS idx_lexeme_pos_confidence
ON linguayi_lexeme(pos_confidence) WHERE pos_confidence < 0.8;

COMMENT ON COLUMN linguayi_lexeme.pos_confidence IS
'Confidence score (0.0-1.0) for part-of-speech tagging; 1.0 = human-verified';

-- ----------------------------------------------------------------------------
-- 3. Morphology Richness Toggle
-- Purpose: Track percentage of possible inflections realized
-- Calculation: (wordforms with morph features) / (total possible wordforms)
-- ----------------------------------------------------------------------------
ALTER TABLE linguayi_lexeme
ADD COLUMN IF NOT EXISTS morphology_richness REAL DEFAULT 0.0;

-- Index for finding well-covered paradigms
CREATE INDEX IF NOT EXISTS idx_lexeme_morphology_richness
ON linguayi_lexeme(morphology_richness) WHERE morphology_richness > 0.0;

COMMENT ON COLUMN linguayi_lexeme.morphology_richness IS
'Percentage (0.0-1.0) of possible inflections with documented morphology';

-- ----------------------------------------------------------------------------
-- 4. Etymology Completeness Toggle
-- Purpose: Track percentage of etymology information filled
-- Fields: source_language, source_word, borrowing_period, notes
-- ----------------------------------------------------------------------------
ALTER TABLE linguayi_lexeme
ADD COLUMN IF NOT EXISTS etymology_completeness REAL DEFAULT 0.0;

-- Index for complete etymologies
CREATE INDEX IF NOT EXISTS idx_lexeme_etymology_completeness
ON linguayi_lexeme(etymology_completeness) WHERE etymology_completeness > 0.0;

COMMENT ON COLUMN linguayi_lexeme.etymology_completeness IS
'Percentage (0.0-1.0) of etymology fields populated (source, period, notes)';

-- ----------------------------------------------------------------------------
-- 5. Last LLM Update Toggle
-- Purpose: Track when lexeme was last enhanced by LLM
-- Use: Avoid redundant LLM queries, track enhancement progress
-- ----------------------------------------------------------------------------
ALTER TABLE linguayi_lexeme
ADD COLUMN IF NOT EXISTS last_llm_update TIMESTAMP WITH TIME ZONE;

COMMENT ON COLUMN linguayi_lexeme.last_llm_update IS
'Timestamp of last LLM enhancement (examples, POS, etymology, etc.)';

-- ----------------------------------------------------------------------------
-- 6. Has Corpus Examples Toggle
-- Purpose: Quick filter for lexemes with real corpus usage
-- Use: Prioritize corpus-backed lexemes for display/analysis
-- ----------------------------------------------------------------------------
ALTER TABLE linguayi_lexeme
ADD COLUMN IF NOT EXISTS has_corpus_examples BOOLEAN DEFAULT FALSE;

CREATE INDEX IF NOT EXISTS idx_lexeme_corpus_examples
ON linguayi_lexeme(has_corpus_examples) WHERE has_corpus_examples = TRUE;

COMMENT ON COLUMN linguayi_lexeme.has_corpus_examples IS
'True if lexeme has verified examples from Yiddish corpus (not just LLM)';

-- ============================================================================
-- HELPER FUNCTIONS FOR CALCULATING COMPLETENESS
-- ============================================================================

-- Calculate morphology richness based on wordform coverage
CREATE OR REPLACE FUNCTION calculate_morphology_richness(lexeme_id UUID)
RETURNS REAL AS $$
DECLARE
    total_wordforms INTEGER;
    analyzed_wordforms INTEGER;
BEGIN
    -- Count total wordforms for this lexeme
    SELECT COUNT(*) INTO total_wordforms
    FROM linguayi_wordform
    WHERE lexeme_id = $1;

    -- If no wordforms, richness is 0
    IF total_wordforms = 0 THEN
        RETURN 0.0;
    END IF;

    -- Count wordforms with morphological features
    SELECT COUNT(*) INTO analyzed_wordforms
    FROM linguayi_wordform
    WHERE lexeme_id = $1
      AND morph_features IS NOT NULL;

    -- Return ratio
    RETURN (analyzed_wordforms::REAL / total_wordforms::REAL);
END;
$$ LANGUAGE plpgsql;

-- Calculate etymology completeness based on filled fields
CREATE OR REPLACE FUNCTION calculate_etymology_completeness(lexeme_id UUID)
RETURNS REAL AS $$
DECLARE
    completeness REAL := 0.0;
    lexeme RECORD;
BEGIN
    SELECT * INTO lexeme FROM linguayi_lexeme WHERE id = $1;

    -- Check which fields are populated
    IF lexeme.etymology_source_language IS NOT NULL THEN
        completeness := completeness + 0.4;
    END IF;

    IF lexeme.etymology_source_word IS NOT NULL THEN
        completeness := completeness + 0.3;
    END IF;

    IF lexeme.etymology_borrowing_period IS NOT NULL THEN
        completeness := completeness + 0.2;
    END IF;

    IF lexeme.etymology_notes IS NOT NULL THEN
        completeness := completeness + 0.1;
    END IF;

    RETURN completeness;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- VIEWS FOR MONITORING TOGGLEABLE FEATURE PROGRESS
-- ============================================================================

-- View: Lexemes ready for LLM enhancement
CREATE OR REPLACE VIEW lexemes_ready_for_enhancement AS
SELECT
    id,
    canonical_hebrew,
    canonical_roman,
    part_of_speech,
    example_count,
    pos_confidence,
    morphology_richness,
    etymology_completeness,
    last_llm_update
FROM linguayi_lexeme
WHERE example_count = 0
   OR pos_confidence < 0.8
   OR morphology_richness < 0.5
   OR etymology_completeness < 0.5
ORDER BY
    (example_count + pos_confidence + morphology_richness + etymology_completeness) ASC;

COMMENT ON VIEW lexemes_ready_for_enhancement IS
'Lexemes that would benefit from LLM enhancement, prioritized by need';

-- View: Enhancement statistics
CREATE OR REPLACE VIEW enhancement_statistics AS
SELECT
    COUNT(*) as total_lexemes,
    COUNT(*) FILTER (WHERE example_count > 0) as with_examples,
    COUNT(*) FILTER (WHERE pos_confidence < 1.0) as with_llm_pos,
    COUNT(*) FILTER (WHERE morphology_richness > 0) as with_morphology,
    COUNT(*) FILTER (WHERE etymology_completeness > 0) as with_etymology,
    COUNT(*) FILTER (WHERE has_corpus_examples = TRUE) as with_corpus,
    AVG(example_count) as avg_examples,
    AVG(pos_confidence) as avg_pos_confidence,
    AVG(morphology_richness) as avg_morphology_richness,
    AVG(etymology_completeness) as avg_etymology_completeness
FROM linguayi_lexeme;

COMMENT ON VIEW enhancement_statistics IS
'Overview of toggleable feature coverage across the lexicon';

-- ============================================================================
-- INITIAL DATA POPULATION
-- ============================================================================

-- Initialize all toggle columns to 0/NULL for existing lexemes
UPDATE linguayi_lexeme
SET
    example_count = COALESCE(example_count, 0),
    pos_confidence = COALESCE(pos_confidence, 1.0),
    morphology_richness = COALESCE(morphology_richness, 0.0),
    etymology_completeness = COALESCE(etymology_completeness, 0.0)
WHERE example_count IS NULL
   OR pos_confidence IS NULL
   OR morphology_richness IS NULL
   OR etymology_completeness IS NULL;

-- ============================================================================
-- GRANT PERMISSIONS
-- ============================================================================

-- Grant select on views for monitoring
GRANT SELECT ON lexemes_ready_for_enhancement TO CURRENT_USER;
GRANT SELECT ON enhancement_statistics TO CURRENT_USER;

-- ============================================================================
-- MIGRATION COMPLETE
-- ============================================================================

-- Migration: 003_toggleable_features.sql
-- Author: Foundation Enhancement
-- Date: 2025-02-05
-- Description: Add incremental feature tracking for LLM-enhanced lexemes
