---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1034
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

TERMINAL

Copy code
Copied!

$ cue import basic.proto

The command creates this CUE file:

Copied!
basic.cue

Copy code
Copied!

// Package basic is rather basic.
package basic

// This is my type.
#MyType: {
	stringValue?: string @protobuf(1,string,name=string_value) // Some string value

	// A method must start with a capital letter.
	method?: [...string] @protobuf(2,string)
	method?: [...=~"^[A-Z]"]
}

Do you notice how the method field’s type and constraints are split over two lines?
That’s exactly as expected, because CUE’s core operation is to automatically
unify each field’s right-hand-side.

The behaviour of the cue import command can be affected by the flags outlined
in the proto mode section of
the command’s help text [/docs/reference/command/cue-help-import/].

USING THE GO API

CUE’s Go API can achieve the same result as the cue import command,
converting Protobuf definitions to CUE, but with
more customization and flexibility [https://pkg.go.dev/cuelang.org/go/encoding/protobuf#Config].

This simple Go code takes the basic.proto file shown above, and prints the
equivalent CUE:

Copied!
main.go

Copy code
Copied!

package main

import (
	"fmt"
	"log"

	"cuelang.org/go/cue/format"
	"cuelang.org/go/encoding/protobuf"
)

func main() {
	file, err := protobuf.Extract("basic.proto", nil, &protobuf.Config{
		Paths: []string{ /* paths to proto includes */ },
	})
	if err != nil {
		log.Fatal(err)
	}
	b, _ := format.Node(file)
	fmt.Println(string(b))
}

Here’s the program’s output:

TERMINAL

Copy code
Copied!

$ go run main.go
// Package basic is rather basic.
package basic

// This is my type.
#MyType: {
	stringValue?: string @protobuf(1,string,name=string_value) // Some string value

	// A method must start with a capital letter.
	method?: [...string] @protobuf(2,string)
	method?: [...=~"^[A-Z]"]
}

Notice that the ouput is identical to the CUE produced previously by cue import.

EXTRACTING CUE FROM SEVERAL PROTOBUF FILES

In some environments it might be necessary to import multiple Protobuf files
