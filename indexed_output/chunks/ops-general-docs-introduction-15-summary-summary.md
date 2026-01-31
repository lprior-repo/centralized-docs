---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#15-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: soft constraints. SEPARATE CONFIGURATION FROM COMPUTATION
---

soft constraints.

SEPARATE CONFIGURATION FROM COMPUTATION

There comes a time that one (seemingly) will need do complex
computations to generate some configuration data.
But simplicity of a configuration language can be paramount when one quickly
needs to make changes.
These are obviously conflicting interests.

CUE takes the stance that computation and configuration should
be separated.
And CUE actually makes this easy.
The data that needs to be computed can be generated outside of CUE
and put in a file that is to be mixed in.
