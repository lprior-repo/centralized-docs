---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1028
summary: 	// going and going past the point where it should probably have. 	children?: [
---

	// going and going past the point where it should probably have
	// stopped.
	children?: [...string]
	"home phone"?: string @deprecated()
	...
}

We use the imported schema to validate some known-good data (good.json)
and known-bad data (bad.json):

Copied!
good.json

Copy code
Copied!

{
    "name": "Dorothy Cartwright",
    "address": "Ripon, North Yorkshire"
}

Copied!
bad.json

Copy code
Copied!

{
    "name": [
        "Charlie",
        "Cartwright"
    ],
    "address": "Ripon, North Yorkshire"
}

The
cue vet [/docs/reference/command/cue-help-vet/]
command validates our data against the #Person constraint:

TERMINAL

Copy code
Copied!

$ cue vet -c -d '#Person' schema.cue good.json bad.json
name: conflicting values ["Charlie","Cartwright"] and strings.MinRunes(1) (mismatched types list and string):
    ./bad.json:2:13
    ./schema.cue:11:9

The cue vet command can also validate the data using the JSON Schema directly:

TERMINAL

Copy code
Copied!

$ cue vet -c schema.json good.json bad.json
name: conflicting values ["Charlie","Cartwright"] and strings.MinRunes(1) (mismatched types list and string):
    ./bad.json:2:13
    ./schema.json:13:13

The cue command normally recognises JSON Schema’s signature fields and treats
the contents of JSON Schema as data constraints - not just additional data.
A qualifier can be used to change this behaviour, as outlined in
cue help filetypes [/docs/reference/command/cue-help-filetypes/]:

TERMINAL

Copy code
Copied!

$ cue def json: schema.json
$schema:     "https://json-schema.org/draft/2020-12/schema"
type:        "object"
title:       "Main Person schema."
description: "This schema defines a person."
...

USING JSON SCHEMA WITH THE GO API

The
encoding/jsonschema [https://pkg.go.dev/cuelang.org/go/encoding/jsonschema]
API
allows you to work with JSON Schema in Go code.

As with the cue command examples shown above, the API can be used to convert
JSON Schema to CUE. However, in this next example, we’ll use the API in a more
fully-formed context: controlling data validation at a lower level.

This Go program validates a JSON data file against a JSON Schema:

Copied!
main.go

Copy code
Copied!

package main

import (
	"flag"
	"fmt"
	"log"
	"os"

	"cuelang.org/go/cue"
	"cuelang.org/go/cue/cuecontext"
	"cuelang.org/go/cue/errors"
	"cuelang.org/go/encoding/json"
	"cuelang.org/go/encoding/jsonschema"
)

func main() {
	log.SetFlags(0)
	flag.Parse()
	args := flag.Args()

	// A cue.Context is used for building/compiling CUE at a low-level.
	// It replaces cue.Runtime.
	ctx := cuecontext.New()

	if len(args) != 2 {
		log.Fatalf("usage:\n\t%s SCHEMA.json DATA.json\n", os.Args[0])
	}

	// Load the schema file JSON
	schemaFile, err := os.ReadFile(args[0])
	if err != nil {
		log.Fatal(err)
	}
	schemaJsonAst, err := json.Extract(args[0], schemaFile)
	if err != nil {
		log.Fatal(err)
	}
	schemaJson := ctx.BuildExpr(schemaJsonAst)

	// Extract JSON Schema from the JSON
	schemaAst, err := jsonschema.Extract(schemaJson, &jsonschema.Config{
		Strict: true,
	})
	if err != nil {
		log.Fatal(err)
	}

	// Build a cue.Value of the schema
	schema := ctx.BuildFile(schemaAst)

	// Load the data file JSON
	dataFile, err := os.ReadFile(args[1])
	if err != nil {
		log.Fatal(err)
	}
	dataAst, err := json.Extract(args[1], dataFile)
	if err != nil {
		log.Fatal(err)
	}

	// Build a cue.Value of the data
	data := ctx.BuildExpr(dataAst)

	// Unify the schema and data
	res := schema.Unify(data)

	// Validate whether the combined (unified) result has errors or not.
	if err := res.Validate(cue.Concrete(true)); err != nil {
		// If errors, report them and fail.
		log.Fatal(errors.Details(err, nil))
	}
	// If no errors, print the data value
	fmt.Printf("%v\n", res)
}

Running the command validates the data file in the second argument against the
JSON schema in the first argument - printing the data if it’s valid and
displaying a validation error otherwise. Here we use it to validate the same
good.json and bad.json files from above:

TERMINAL

Copy code
Copied!

$ go run . schema.json good.json
{
	name:    "Dorothy Cartwright"
