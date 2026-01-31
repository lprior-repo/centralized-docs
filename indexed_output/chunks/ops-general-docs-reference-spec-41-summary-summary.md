---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#41-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 141
summary: whereby multiple disjuncts can be marked as default. A marked disjunction is one where any of its terms are marked
---

whereby multiple disjuncts can be marked as default.
A marked disjunction is one where any of its terms are marked.
So a | b | *c | d is a single marked disjunction of four terms,
whereas a | (b | *c | d) is an unmarked disjunction of two terms,
one of which is a marked disjunction of three terms.
During unification, if all the marked disjuncts of a marked disjunction are
eliminated, then the remaining unmarked disjuncts are considered as if they
originated from an unmarked disjunction

As explained below, distinguishing the nesting of disjunctions like this
