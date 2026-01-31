---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#3-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 526
summary: making them available as the #Pet schema:. $ cue vet -c api
---

making them available as the #Pet schema:

TERMINAL

Copy code
Copied!

$ cue vet -c api.pet.yaml jonathan.yml -d '#Pet'
kind: 3 errors in empty disjunction:
kind: conflicting values "cat" and "tortoise":
    ./api.pet.yaml:11:7
    ./api.pet.yaml:26:11
    ./jonathan.yml:2:7
kind: conflicting values "dog" and "tortoise":
    ./api.pet.yaml:26:11
    ./jonathan.yml:2:7
kind: conflicting values "goldfish" and "tortoise":
    ./api.pet.yaml:26:11
    ./jonathan.yml:2:7

Perhaps our #Pet schema should be updated to handle more animal types!

USING OPENAPI WITH THE GO API

CUE can also access and generate OpenAPI through its
encoding/openapi [https://pkg.go.dev/cuelang.org/go/encoding/openapi]
Go API.

Generating an OpenAPI definition can be as simple as this:

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
		log.Fatal(err)
	}

	// Render as indented JSON
	b, err := json.MarshalIndent(topValue, "", "  ")
	if err != nil {
		log.Fatal(err)
	}
	b = append(b, '\n')
	os.Stdout.Write(b)
}

Running this code successfully expresses the constraints in our original
schema.cue file as an OpenAPI document:

TERMINAL

Copy code
Copied!

$ go run .
{
  "openapi": "3.0.0",
  "info": {
    "title": "A schema for the pet API.",
    "version": "v1.2.3"
  },
  "paths": {},
  "components": {
    "schemas": {
      "Kind": {
...

The encoding/openapi [https://pkg.go.dev/cuelang.org/go/encoding/openapi]
package provides options to make a definition self-contained, to filter
constraints, and so on. The expanding references option enables the
“Structural OpenAPI” form required by CRDs targeting Kubernetes version 1.15
