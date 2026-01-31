---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 522
summary: # How CUE works with YAML | CUE. **Source:** https://cuelang
---

# How CUE works with YAML | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-yaml/

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


 2. HOW CUE WORKS WITH YAML

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

READING AND WRITING YAML

The cue tool natively supports reading and writing YAML files, including
those containing multiple documents.

This allows YAML files to be processed by CUE’s wide range of data, schema, and
policy validation capabilities, and to convert input formats to YAML - as
demonstrated here by
cue export [/docs/reference/command/cue-help-export/]
unifying all its YAML, JSON, and CUE input files as YAML:

 * 
   
   Copied!
   data.yml
 * 
   
   Copied!
   data.json
 * 
   
   Copied!
   data.cue

Copy code
Copied!

a: 1
b: "2"
c: "three"
d: 4.4


Copy code
Copied!

{
    "e": 5,
    "f": "6"
}


Copy code
Copied!

g: "seven"
h: 4.4 * 2

TERMINAL

Copy code
Copied!

$ cue export --out yaml data.yml data.json data.cue
a: 1
b: "2"
c: three
d: 4.4
e: 5
"f": "6"
g: seven
h: 8.8

In addition to YAML, cue can read and write
a range of other formats [/docs/integration/].

VALIDATING YAML FILES AGAINST A SCHEMA

CUE is often used to make systems safer without having to teach the underlying
system components about CUE. Because the cue tool can validate YAML files
