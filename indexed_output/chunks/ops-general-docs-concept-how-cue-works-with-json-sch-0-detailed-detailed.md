---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1024
summary: # How CUE works with JSON Schema | CUE. **Source:** https://cuelang
---

# How CUE works with JSON Schema | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-json-schema/

Skip to content

Homepage of CUE [/]
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]

 * 
   GitHub [https://github.com/cue-lang/cue]
 * 
   Slack [/s/slack]
 * 
   Discord [/s/discord]
 * 
   X (Twitter) [https://twitter.com/cue_lang]
 * 
   Bluesky [https://bsky.app/profile/cuelang.org]
 * 
   YouTube [https://www.youtube.com/@cuelang/videos]

Install
[/docs/introduction/installation/]

Search [/search]

What are you looking for?

Menu

 1. Concept Guides [https://cuelang.org/docs/concept/]


 2. HOW CUE WORKS WITH JSON SCHEMA

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
myitcv [https://github.com/myitcv.png]
Paul Jolly
myitcv [https://github.com/myitcv.png]
Paul Jolly

Github profile

[https://github.com/myitcv]

Search all content by this author

[/search/?q=author:myitcv]
 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]
 * go api [/search?q=tag:%22go%20api%22]

CUE has first class support for JSON Schema [https://json-schema.org/]:
both the cue command and the Go API understand the format.

Constraints stored as JSON Schema are available for cue commands to use as if
they were expressed in CUE.
This allows you to work with JSON Schema constraints directly, using them to
validate data, and to represent them natively in CUE’s more succinct and
expressive form.

In this guide we’ll see:

 * cue import [/docs/reference/command/cue-help-import/] converting a
   JSON Schema to CUE,
 * cue vet [/docs/reference/command/cue-help-vet/] using JSON Schema
   constraints directly,
 * and the
   encoding/jsonschema [https://pkg.go.dev/cuelang.org/go/encoding/jsonschema]
   Go API validating data against a JSON Schema.

The ability to export CUE constraints as JSON Schema is tracked in issue #929 [/issue/929].

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
