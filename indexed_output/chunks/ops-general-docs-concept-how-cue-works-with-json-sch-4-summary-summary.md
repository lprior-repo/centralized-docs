---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#4-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: USING JSON SCHEMA WITH THE CUE COMMAND. The cue import [/docs/reference/command/cue-help-import/] command can
---


USING JSON SCHEMA WITH THE CUE COMMAND

The cue import [/docs/reference/command/cue-help-import/] command can
produce CUE from JSON Schema.

Let’s start with this JSON Schema:

Copied!
schema.json

Copy code
Copied!

{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "title": "Main Person schema.",
    "description": "This schema defines a person.",
    "required": [
        "name"
    ],
    "properties": {
        "name": {
            "description": "What is this person called?",
