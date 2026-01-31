---
doc_id: ops/general/docs-concept-how-cue-works-with-toml
chunk_id: ops/general/docs-concept-how-cue-works-with-toml#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 522
summary: # How CUE works with TOML | CUE. **Source:** https://cuelang
---

# How CUE works with TOML | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-toml/

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


 2. HOW CUE WORKS WITH TOML

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

READING AND WRITING TOML

The cue command natively supports reading and writing TOML files and data.
TOML can be processed by CUE’s wide range of data, schema, and policy
validation capabilities.
Data in any supported encoding can be read and exported as TOML
– as demonstrated here by
cue export [/docs/reference/command/cue-export/]
unifying its TOML, JSON, and CUE input files and producing TOML:

 * 
   
   Copied!
   a.toml
 * 
   
   Copied!
   b.json
 * 
   
   Copied!
   c.cue

Copy code
Copied!

a = "1"

[b]
c = 2.2

[b.d]
e = 3


Copy code
Copied!

{
    "f": "4",
    "g": 5.5
}


Copy code
Copied!

b: _
g: _

h: "six"
b: d: i: g + b.d.e

TERMINAL

Copy code
Copied!

$ cue export --out toml a.toml b.json c.cue
a = '1'
f = '4'
g = 5.5
h = 'six'

[b]
c = 2.2

[b.d]
e = 3
i = 8.5

The cue command can read and write
a range of other formats [/docs/integration/]
as well as TOML.

VALIDATING TOML FILES AGAINST A SCHEMA

CUE is often used to make systems safer without having to teach the underlying
system components about CUE. Because the cue tool can validate TOML files
