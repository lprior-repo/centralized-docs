---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: $ cue export.         \"version\": \"1
---


$ cue export
{
    "info": {
        "version": "1.42.0",
        "source": "A bar"
    }
}

File embedding is available from CUE v0.12.0 onwards.
Find out more about this powerful validation feature in
Embedding files in a CUE evaluation [/docs/howto/embed-files-in-cue-evaluation/].

ENCODING JSON INSIDE CUE

CUE is frequently used to generate configuration files. Some systems allow
their configuration files to contain JSON encoded in string fields,
irrespective of the file’s main data format.

CUE’s standard library provides
