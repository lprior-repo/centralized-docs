---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#31-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: The whitespace before a closing triple quote must appear before any non-empty. line after the opening quote and will be removed from each of these
---

The whitespace before a closing triple quote must appear before any non-empty
line after the opening quote and will be removed from each of these
lines in the string literal.
A closing triple quote may not appear in the string.
To include it is suffices to escape one of the quotes.


Copy code
Copied!

"""
    lily:
    out of the water
    out of itself

    bass
    picking \
    bugs
    off the moon
        — Nick Virgilio, Selected Haiku, 1988
    """

This represents the same string as:


Copy code
