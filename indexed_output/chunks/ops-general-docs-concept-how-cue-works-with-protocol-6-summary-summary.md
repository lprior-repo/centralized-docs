---
doc_id: ops/general/docs-concept-how-cue-works-with-protocol
chunk_id: ops/general/docs-concept-how-cue-works-with-protocol#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: equivalent CUE:. package main
---

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
