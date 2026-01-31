---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 525
summary: # The Logic of CUE | CUE. **Source:** https://cuelang
---

# The Logic of CUE | CUE

**Source:** https://cuelang.org/docs/concept/the-logic-of-cue/

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


 2. THE LOGIC OF CUE

This page explains the core concept on which pretty much everything that is CUE
depends.
It helps to get a top-down understanding and frame of reference,
but it is not necessary for learning the language.

TYPES ARE VALUES

There are two core aspects of CUE that make it different from
the usual programming or configuration languages:

 * Types are values
 * Values (and thus types) are ordered into a lattice

These properties are relevant almost to everything that makes CUE what it is.
They simplify the language, as many concepts that are distinct in other
languages fold together.
The resulting order independence
simplifies reasoning about values for both humans and machines.

It also forces formal rigor on the language, such as defining
exactly what it means to be optional, a default value, or null.
Making sure all values fit in a value lattice leaves no wiggle room.

Finally, the combination of all this allows for many unique features,
for instance:

 * a single language for specifying data, schema, validation
   and policy constraints,
 * meta reasoning, such as determining whether
   a new schema version is backwards compatible,
 * automated rewriting, such as is done by cue trim,
 * creating multi-source constraint pipelines, retaining documentation
   across normalization,

and so on.

THE VALUE LATTICE

Every value in CUE, including what would in most programming languages
be considered types, is partially ordered in a single hierarchy
