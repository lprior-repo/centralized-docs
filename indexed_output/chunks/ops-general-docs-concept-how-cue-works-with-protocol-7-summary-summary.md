---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 141
summary: // This is my type. 	stringValue?: string @protobuf(1,string,name=string_value) // Some string value
---


// This is my type.
#MyType: {
	stringValue?: string @protobuf(1,string,name=string_value) // Some string value

	// A method must start with a capital letter.
	method?: [...string] @protobuf(2,string)
	method?: [...=~"^[A-Z]"]
}

Notice that the ouput is identical to the CUE produced previously by cue import.

EXTRACTING CUE FROM SEVERAL PROTOBUF FILES

In some environments it might be necessary to import multiple Protobuf files
that map to different CUE packages within the same module.
If several .proto files import each other, and other centralized schema
