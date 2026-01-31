---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 530
summary: # How CUE works with JSON Schema | CUE. **Source:** https://cuelang
---

# How CUE works with JSON Schema | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-json-schema/

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


 2. HOW CUE WORKS WITH JSON SCHEMA

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
 * go api [/search?q=tag:%22go%20api%22]

CUE has first class support for JSON Schema [https://json-schema.org/]:
both the cue command and the Go API understand the format.

Constraints stored as JSON Schema are available for cue commands to use as if
they were expressed in CUE.
This allows you to work with JSON Schema constraints directly, using them to
validate data, and to represent them natively in CUE’s more succinct and
expressive form.

In this guide we’ll see:

 * cue import [/docs/reference/command/cue-help-import/] converting a
   JSON Schema to CUE,
 * cue vet [/docs/reference/command/cue-help-vet/] using JSON Schema
   constraints directly,
 * and the
   encoding/jsonschema [https://pkg.go.dev/cuelang.org/go/encoding/jsonschema]
   Go API validating data against a JSON Schema.

The ability to export CUE constraints as JSON Schema is tracked in issue #929 [/issue/929].
