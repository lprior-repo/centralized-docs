---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#4-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: The command creates this CUE file:. // Package basic is rather basic
---


The command creates this CUE file:

Copied!
basic.cue

Copy code
Copied!

// Package basic is rather basic.
package basic

// This is my type.
#MyType: {
	stringValue?: string @protobuf(1,string,name=string_value) // Some string value

	// A method must start with a capital letter.
	method?: [...string] @protobuf(2,string)
	method?: [...=~"^[A-Z]"]
}

Do you notice how the method field’s type and constraints are split over two lines?
That’s exactly as expected, because CUE’s core operation is to automatically
