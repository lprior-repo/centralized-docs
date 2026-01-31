---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#13-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: package main. 	\"encoding/json\"
---


Copied!
main.go

Copy code
Copied!

package main

import (
	"encoding/json"
	"log"
	"os"

	"cuelang.org/go/cue/cuecontext"
	"cuelang.org/go/cue/load"
	"cuelang.org/go/encoding/openapi"
)

func main() {
	ctx := cuecontext.New()
	insts := load.Instances([]string{"schema.cue"}, nil)
	v := ctx.BuildInstance(insts[0])

	// Generate the OpenAPI schema from the value loaded from schema.cue
	f, err := openapi.Generate(v, nil)
	if err != nil {
		log.Fatal(err)
	}
	topValue := ctx.BuildFile(f)
	if err := topValue.Err(); err != nil {
