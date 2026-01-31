---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 144
summary: constraints are combined from different sources:.  * Data validation: different departments or groups can each
---

constraints are combined from different sources:

 * Data validation: different departments or groups can each
   define their own constraints to apply to the same set of data.

 * Code extraction and generation: extract CUE definitions from
   multiple sources (Go code, Protobuf), combine them into a single
   definition, and use that to generate definitions in another
   format (e.g. OpenAPI).

 * Configuration: values can be combined from different sources
   without one having to import the other.

The ordering of values also allows set containment analysis of entire
