---
doc_id: ops/general/docs-tour-types
chunk_id: ops/general/docs-tour-types#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 515
summary: # Type Hierarchy | CUE. **Source:** https://cuelang
---

# Type Hierarchy | CUE

**Source:** https://cuelang.org/docs/tour/types/

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
 2. Types and Values [https://cuelang.org/docs/tour/types/]


 3. TYPE HIERARCHY

CUE defines the following type hierarchy:

_
_|_
{...}
[...]
null
bool
string
bytes
number
int
float
CUE's predefined type hierarchy

CUE defines the value top (or any),
written “_”, such that all types are an instance of top,
and the value bottom (or error),
written “_|_”,
which is an instance of all types.

We can mix the terms types and values interchangeably because
CUE doesn’t distinguish between
types and values [/docs/tour/basics/types-are-values/].
The term “type” merely refers to the kind of a value,
which may or may not be a concrete instance.

In the following hypothetical example, point defines an arbitrary point,
while xaxis and yaxis define any point on their respective axes.
We say that point, xaxis, and yaxis are incomplete,
as they don’t contain values that specify a specific point.
Incomplete values cannot be included when exporting to formats that can’t
represent them.
JSON and YAML, for example, have no way to represent the type number.

Copied!
file.cue

Copy code
Copied!

point: {
	x: number
	y: number
}

xaxis: point
xaxis: y: 0

yaxis: point
yaxis: x: 0

origin: xaxis & yaxis

TERMINAL

Copy code
Copied!

$ cue eval file.cue
point: {
    x: number
    y: number
}
xaxis: {
    x: number
    y: 0
}
yaxis: {
    x: 0
    y: number
}
origin: {
    x: 0
    y: 0
}

By contrast, origin is not incomplete, as it contains only concrete values.
