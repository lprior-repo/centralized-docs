---
doc_id: ops/general/docs-tour-basics
chunk_id: ops/general/docs-tour-basics#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 513
summary: # JSON Superset | CUE. **Source:** https://cuelang
---

# JSON Superset | CUE

**Source:** https://cuelang.org/docs/tour/basics/

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

 1. Tour [https://cuelang.org/docs/tour/]
 2. The Basics of CUE [https://cuelang.org/docs/tour/basics/]


 3. JSON SUPERSET

In its simplest form, CUE looks a lot like JSON.
This is because CUE is a superset of JSON.
Or, put differently: all valid JSON is CUE (but not vice versa).

CUE significantly reduces the pain of dealing with JSON
by introducing several conveniences, including:

 * C-style comments are allowed
 * field names without special characters don’t need to be quoted
 * commas after a field are optional (and are usually omitted)
 * commas after the final element of a list are allowed
 * the outermost curly braces in a CUE file are optional

JSON objects are called structs or maps in CUE.
JSON arrays are called lists
Object members are called fields, which link their name, or label, to a value.

Throughout this tour there are examples that show
some CUE,
a command that processes it,
and then the command’s output
- with each pane featuring a mouseover button that copies the related text to
your clipboard.
Here’s an example that uses cue export to turn file.cue into JSON.


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
