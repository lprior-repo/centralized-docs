---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 146
summary: Search all content by this author. [/search/?q=author:myitcv]
---


Search all content by this author

[/search/?q=author:myitcv]
 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]

CUE has first class support for OpenAPI data schemas: the cue command
automatically recognises OpenAPI by its signature fields, and the Go API has
packages dedicated to the format.
Specifically, CUE supports the
OpenAPI 3.0.0 standard [https://github.com/OAI/OpenAPI-Specification/tree/3.0.0]
through its components.schemas namespace for data schemas.

Constraints stored as OpenAPI data schemas are available for cue commands to
