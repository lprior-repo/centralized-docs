---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: language-neutral, platform-neutral, and extensible mechanism for serializing. structured data, initially developed and released by Google
---

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
