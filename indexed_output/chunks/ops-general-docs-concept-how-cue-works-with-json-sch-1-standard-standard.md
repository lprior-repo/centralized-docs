---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 513
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
            "type": "string",
            "minLength": 1
        },
        "address": {
            "description": "Where does this person live?",
            "type": "string",
            "minLength": 1,
            "maxLength": 200
        },
        "children": {
            "description": "This is a very long comment for some reason, which will keep going and going past the point where it should probably have stopped.",
            "type": "array",
            "items": {
                "type": "string"
            },
            "default": null
        },
        "home phone": {
            "type": "string",
            "deprecated": true
        }
    }
}

We use cue import to convert the JSON Schema to CUE:

TERMINAL

Copy code
Copied!

$ cue import -l '#Person:' schema.json

cue import recognises JSON Schema from its signature fields, and uses the
schema’s constraints to create a shorter, more readable CUE representation.
Our -l parameter tells cue to place the constraints inside the #Person
definition:

Copied!
schema.cue

Copy code
Copied!

// Main Person schema.
//
// This schema defines a person.

import "strings"

#Person: {
	@jsonschema(schema="https://json-schema.org/draft/2020-12/schema")

	// What is this person called?
	name!: strings.MinRunes(1)

	// Where does this person live?
	address?: strings.MinRunes(1) & strings.MaxRunes(200)

	// This is a very long comment for some reason, which will keep
	// going and going past the point where it should probably have
	// stopped.
