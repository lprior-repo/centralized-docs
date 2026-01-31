---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 522
summary: # How CUE works with OpenAPI | CUE. **Source:** https://cuelang
---

# How CUE works with OpenAPI | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-openapi/

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


 2. HOW CUE WORKS WITH OPENAPI

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
myitcv [https://github.com/myitcv.png]
Paul Jolly
myitcv [https://github.com/myitcv.png]
Paul Jolly

Github profile

[https://github.com/myitcv]

Search all content by this author

[/search/?q=author:myitcv]
 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]

CUE has first class support for OpenAPI data schemas: the cue command
automatically recognises OpenAPI by its signature fields, and the Go API has
packages dedicated to the format.
Specifically, CUE supports the
OpenAPI 3.0.0 standard [https://github.com/OAI/OpenAPI-Specification/tree/3.0.0]
through its components.schemas namespace for data schemas.

Constraints stored as OpenAPI data schemas are available for cue commands to
use as if they were expressed in CUE. This allows you to work with these
constraints directly, using them to validate data, and to represent them
natively in CUE’s significantly more concise form.

In this guide, we’ll see:

 * cue def [/docs/reference/command/cue-help-def/]
   generating an OpenAPI data schema from a CUE definition,
 * cue import [/docs/reference/command/cue-help-import/]
   turning the generated OpenAPI back into CUE,
