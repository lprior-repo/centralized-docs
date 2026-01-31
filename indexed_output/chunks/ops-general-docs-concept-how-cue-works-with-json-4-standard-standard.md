---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#4-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 519
summary: item: [string]: json. Validate(#Dimensions)
---

item: [string]: json.Validate(#Dimensions)

// bed is correctly specified.
item: bed: #"{ "width": 2, "height": 0.1, "depth": 2 }"#
// table's width is incorrectly specified as a string.
item: table: #"{ "width": "34", "height": 23, "depth": 0.2 }"#
// painting's height field name is incorrectly upper-cased.
item: painting: #"{ "width": 34, "HEIGHT": 12, "depth": 0.2 }"#

TERMINAL

Copy code
Copied!

$ cue vet -c furniture.cue
item.painting: invalid value "{ \"width\": 34, \"HEIGHT\": 12, \"depth\": 0.2 }" (does not satisfy encoding/json.Validate): error in call to encoding/json.Validate: field not allowed:
    ./furniture.cue:10:17
    ./furniture.cue:17:17
    json.Validate:1:16
item.table: invalid value "{ \"width\": \"34\", \"height\": 23, \"depth\": 0.2 }" (does not satisfy encoding/json.Validate): error in call to encoding/json.Validate: conflicting values "34" and number (mismatched types string and number):
    ./furniture.cue:10:17
    ./furniture.cue:4:10
    ./furniture.cue:15:14
    json.Validate:1:1
    json.Validate:1:12

OTHER JSON FUNCTIONS

The
json package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/json]
contains other useful functions, including those that format JSON specifically
for humans to read, or for machines to consume. These functions are
demonstrated in guides that you can discover through the site’s search page:
🔍 
search for how-to guides mentioning “encoding/json” [/search/?q=encoding/json%20contentType:%22How-to%20Guides%22]

CONVERTING JSON FILES TO CUE

Because
every valid JSON file is also a CUE file [/docs/tour/basics/json-superset/],
one very easy way to convert JSON files to CUE is simply to rename them from
.json to .cue!

In more complex situations
cue import [/docs/reference/command/cue-help-import/]
can create a CUE file for each JSON file it’s given, and can even recognise
encoded YAML and JSON fields, and convert those structures recursively.

Examples of this command being used can be found in the
cue import CLI reference documentation [/docs/reference/command/cue-help-import/].
