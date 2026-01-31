---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#52-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: representation with limited precision. That said, every implementation must:
---

representation with limited precision.
That said, every implementation must:

 * Represent integer values with at least 256 bits.
 * Represent floating-point values with a mantissa of at least 256 bits and
   a signed binary exponent of at least 16 bits.
 * Give an error if unable to represent an integer value precisely.
 * Give an error if unable to represent a floating-point value due to overflow.
 * Round to the nearest representable value if unable to represent
   a floating-point value due to limits on precision.
