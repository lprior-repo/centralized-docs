---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 513
summary: # How CUE works with Go | CUE. **Source:** https://cuelang
---

# How CUE works with Go | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-go/

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


 2. HOW CUE WORKS WITH GO

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
 * go api [/search?q=tag:%22go%20api%22]

CUE is designed to complement and work with the Go programming language.
It offers a powerful API that enables Go code to take advantage of CUE’s
advanced capabilites.
Additionally, CUE makes it easy to use Go as your source of truth by using the
cue command to convert Go types to CUE.

In this guide we’ll demonstrate importing some Kubernetes API code to generate
CUE schemas. We’ll also use the API to convert both CUE and non-CUE data to
native Go values, and validate some Go data natively with CUE.

CONVERTING GO TYPES TO CUE

If you’ve already invested time in developing Go types, you might need them to
be the source of truth in your system whilst also wanting to validate data that
matches those types against the more detailed constraints that CUE allows.

The cue command can help you achieve this as it can convert arbitrary Go types to CUE.
