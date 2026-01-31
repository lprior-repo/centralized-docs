---
doc_id: ops/general/docs-tour-basics
chunk_id: ops/general/docs-tour-basics#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: import \"math\". // Simple labels don't need to be quoted
---



Copied!
file.cue

Copy code
Copied!

import "math"

// Simple labels don't need to be quoted.
one:       1
two:       2
piPlusOne: math.Pi + 1

// Field names must be quoted if they contain
// special characters, such as hyphen and space.
"quoted field names": {
	"two-and-a-half":    2.5
	"three point three": 3.3
	"four^four":         math.Pow(4, 4)
}

aList: [
	1,
	2,
	3,
]

TERMINAL

Copy code
Copied!

$ cue export file.cue --out json
{
    "one": 1,
    "two": 2,
    "piPlusOne": 4.141592653589793238462643383279503,
