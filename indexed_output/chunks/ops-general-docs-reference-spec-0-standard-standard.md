---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 519
summary: # The CUE Language Specification | CUE. **Source:** https://cuelang
---

# The CUE Language Specification | CUE

**Source:** https://cuelang.org/docs/reference/spec/

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

 1. References [https://cuelang.org/docs/reference/]


 2. THE CUE LANGUAGE SPECIFICATION

mpvl [https://github.com/mpvl.png]
Marcel van Lohuizen
mpvl [https://github.com/mpvl.png]
Marcel van Lohuizen

Github profile

[https://github.com/mpvl]

Search all content by this author

[/search/?q=author:mpvl]


NOTE TO IMPLEMENTORS

Notes on the formalism underlying this specification can be found
here [https://github.com/cue-lang/cue/blob/master/doc/ref/impl.md].

INTRODUCTION

This is a reference manual for the CUE data constraint language.
CUE, pronounced cue or Q, is a general-purpose and strongly typed
constraint-based language.
It can be used for data templating, data validation, code generation, scripting,
and many other applications involving structured data.
The CUE tooling, layered on top of CUE, provides
a general purpose scripting language for creating scripts as well as
simple servers, also expressed in CUE.

CUE was designed with cloud configuration and related systems in mind,
but is not limited to this domain.
It derives its formalism from relational programming languages.
This formalism allows for managing and reasoning over large amounts of
data in a straightforward manner.

The grammar is compact and regular, allowing for easy analysis by automatic
tools such as integrated development environments.

This document is maintained by mpvl@golang.org [mpvl@golang.org].
CUE has a lot of similarities with the Go language. This document draws heavily
from the Go specification as a result.
