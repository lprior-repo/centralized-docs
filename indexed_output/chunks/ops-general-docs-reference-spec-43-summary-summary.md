---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#43-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 138
summary: The rewrite rules for unifying such values are as follows:. U0: ⟨v1⟩ & ⟨v2⟩         => ⟨v1&v2⟩
---

The rewrite rules for unifying such values are as follows:


Copy code
Copied!

U0: ⟨v1⟩ & ⟨v2⟩         => ⟨v1&v2⟩
U1: ⟨v1, d1⟩ & ⟨v2⟩     => ⟨v1&v2, d1&v2⟩
U2: ⟨v1, d1⟩ & ⟨v2, d2⟩ => ⟨v1&v2, d1&d2⟩

The rewrite rules for disjoining terms of unmarked disjunctions are


Copy code
Copied!

D0: ⟨v1⟩ | ⟨v2⟩         => ⟨v1|v2⟩
D1: ⟨v1, d1⟩ | ⟨v2⟩     => ⟨v1|v2, d1⟩
D2: ⟨v1, d1⟩ | ⟨v2, d2⟩ => ⟨v1|v2, d1|d2⟩

Terms of marked disjunctions are first rewritten according to the following
