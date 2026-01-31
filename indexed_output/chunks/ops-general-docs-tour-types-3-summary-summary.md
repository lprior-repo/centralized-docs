---
doc_id: ops/general/docs-tour-types
chunk_id: ops/general/docs-tour-types#3-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: represent them. JSON and YAML, for example, have no way to represent the type number
---

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
However, notice that we didn’t need to specify its values explicitly.
