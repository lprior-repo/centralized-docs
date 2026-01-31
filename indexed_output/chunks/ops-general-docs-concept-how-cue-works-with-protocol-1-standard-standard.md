---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 514
summary: $ cue import basic. The command creates this CUE file:
---


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
