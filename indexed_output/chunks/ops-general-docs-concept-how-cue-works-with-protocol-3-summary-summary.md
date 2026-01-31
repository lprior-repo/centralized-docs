---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: // Package basic is rather basic. package cuelang
---


// Package basic is rather basic.
package cuelang.examples.basic;

import "cue/cue.proto";

option go_package = "cuelang.org/encoding/protobuf/examples/basic";

// This is my type.
message MyType {
    string string_value = 1; // Some string value

    // A method must start with a capital letter.
    repeated string method = 2 [(cue.val) = '[...=~"^[A-Z]"]'];
}

The cue import command converts Protobuf to CUE.
It indicates success by displaying no output:

TERMINAL

Copy code
Copied!

$ cue import basic.proto
