---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: // Main Person schema. // This schema defines a person
---


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
	children?: [...string]
