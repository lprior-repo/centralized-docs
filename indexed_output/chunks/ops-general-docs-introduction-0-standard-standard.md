---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 514
summary: # Introduction | CUE. **Source:** https://cuelang
---

# Introduction | CUE

**Source:** https://cuelang.org/docs/introduction/

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

 1. INTRODUCTION

WELCOME!

CUE is an open-source data validation language and inference engine
with its roots in logic programming.
Although the language is not a general-purpose programming language,
it has many applications, such as
data validation, data templating, configuration, querying,
code generation and even scripting.
The inference engine can be used to validate
data in code or to include it as part of a code generation pipeline.

A key thing that sets CUE apart from its peer languages
is that it merges types and values into a single concept.

Whereas in most languages types and values are strictly distinct,
CUE orders them in a single hierarchy (a lattice, to be precise).
This is a very powerful concept that allows CUE to do
many fancy things.
It also simplifies matters.
For instance, there is no need for generics, and enums, sum types
and null coalescing are all the same thing.

APPLICATIONS

CUE’s design ensures that combining CUE values in any
order always gives the same result
(it is associative, commutative and idempotent).
This makes CUE particularly well-suited for cases where CUE
constraints are combined from different sources:

 * Data validation: different departments or groups can each
   define their own constraints to apply to the same set of data.

 * Code extraction and generation: extract CUE definitions from
   multiple sources (Go code, Protobuf), combine them into a single
   definition, and use that to generate definitions in another
   format (e.g. OpenAPI).
