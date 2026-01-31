---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 514
summary: # How CUE works with JSON | CUE. **Source:** https://cuelang
---

# How CUE works with JSON | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-json/

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


 2. HOW CUE WORKS WITH JSON

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]

READING AND WRITING JSON

CUE is a superset of JSON [/docs/tour/basics/json-superset/].
In other words: all valid JSON is CUE.

The cue tool natively supports reading and writing JSON files. In fact, JSON
is its default output format.

This allows JSON files to be processed by CUE’s wide range of data, schema, and
policy validation capabilities, and to convert input formats to JSON - as
demonstrated here by
cue export [/docs/reference/command/cue-help-export/]
unifying all its JSON, YAML, and CUE input files as JSON:

 * 
   
   Copied!
   data.json
 * 
   
   Copied!
   data.yml
 * 
   
   Copied!
   data.cue

Copy code
Copied!

{
    "a": 1,
    "b": "2",
    "c": "three",
    "d": 4.4
}


Copy code
Copied!

e: 5
f: "6"


Copy code
Copied!

g: "seven"
h: 4.4 * 2

TERMINAL

Copy code
Copied!

$ cue export data.json data.yml data.cue
{
    "a": 1,
    "b": "2",
    "c": "three",
    "d": 4.4,
    "e": 5,
    "f": "6",
    "g": "seven",
    "h": 8.8
}

In addition to JSON, cue can read and write
a range of other formats [/docs/integration/].
