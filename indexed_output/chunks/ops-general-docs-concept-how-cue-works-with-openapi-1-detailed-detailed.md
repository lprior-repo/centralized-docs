---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#1-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1033
summary:       description: A Pet is a pet that we handle.       type: object
---

      enum:
        - cat
        - dog
        - goldfish
    Pet:
      description: A Pet is a pet that we handle.
      type: object
      required:
        - name
        - kind
      properties:
        name:
          description: A pet has a name.
          type: string
        kind:
          $ref: '#/components/schemas/Kind'
        age:
          description: Centenarian pets are not handled.
          type: integer
          minimum: 0
          maximum: 100
          exclusiveMaximum: true

Because CUE is more expressive than OpenAPI,
it isn’t possible to generate a precise OpenAPI equivalent for every CUE constraint.
CUE does the best conversion it can,
limited by what OpenAPI’s data schemas can represent.

The cue import [/docs/reference/command/cue-help-import/] command
can take constraints found in OpenAPI data schemas and express them as CUE.
Here we take the definition produced by cue def, above, and convert it back to CUE:

TERMINAL

Copy code
Copied!

$ cue import -p api api.pet.yaml

This produces the following CUE, which is as close to the original schema.cue
as OpenAPI currently permits:

Copied!
api.pet.cue

Copy code
Copied!

// A schema for the pet API.
package api

info: {
	title:   *"A schema for the pet API." | string
	version: *"v1.2.3" | string
}
// Kind encodes the different pets we handle.
#Kind: "cat" | "dog" | "goldfish"

// A Pet is a pet that we handle.
#Pet: {
	// A pet has a name.
	name!: string
	kind!: #Kind

	// Centenarian pets are not handled.
	age?: int & >=0 & <100
	...
}

The cue vet [/docs/reference/command/cue-help-vet/] command can
directly use constraints from OpenAPI data schemas to validate data.

Let’s check the details of a well-known animal,
Jonathan the tortoise [https://en.wikipedia.org/wiki/Jonathan_%28tortoise%29]:

Copied!
jonathan.yml

Copy code
Copied!

name: Jonathan
kind: tortoise
website: https://en.wikipedia.org/wiki/Jonathan_(tortoise)

Let’s validate Jonathan against the contents of our OpenAPI document, api.pet.yaml.
CUE automatically recognises the constraints in the components.schemas namespace,
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
