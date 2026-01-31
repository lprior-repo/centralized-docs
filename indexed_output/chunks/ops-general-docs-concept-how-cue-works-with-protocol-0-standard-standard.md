---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#0-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 520
summary: # How CUE works with Protocol Buffers | CUE. **Source:** https://cuelang
---

# How CUE works with Protocol Buffers | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-protocol-buffers/

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


 2. HOW CUE WORKS WITH PROTOCOL BUFFERS

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
 * encodings [/search?q=tag:encodings]
 * go api [/search?q=tag:%22go%20api%22]

Protocol Buffers [https://protobuf.dev/], also known as Protobuf, is a
language-neutral, platform-neutral, and extensible mechanism for serializing
structured data, initially developed and released by Google.

Protobuf definitions can be converted to CUE by the cue command and CUE’s Go
API, promoting any CUE validation code placed in Protobuf options to
first-class CUE value constraints.

USING THE CUE COMMAND

Let’s start by converting Protobuf to CUE using the cue command.
We’ll begin with this Protobuf file, basic.proto:

Copied!
basic.proto

Copy code
Copied!

syntax = "proto3";

// Package basic is rather basic.
package cuelang.examples.basic;

import "cue/cue.proto";

option go_package = "cuelang.org/encoding/protobuf/examples/basic";

// This is my type.
message MyType {
    string string_value = 1; // Some string value

    // A method must start with a capital letter.
    repeated string method = 2 [(cue.val) = '[...=~"^[A-Z]"]'];
}

The cue import command converts Protobuf to CUE.
It indicates success by displaying no output:
