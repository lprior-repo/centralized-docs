---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#1-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 514
summary:  * cue vet [/docs/reference/command/cue-help-vet/].    using an OpenAPI data schema directly, to validate some data,
---

 * cue vet [/docs/reference/command/cue-help-vet/]
   using an OpenAPI data schema directly, to validate some data,
 * and the
   encoding/openapi [https://pkg.go.dev/cuelang.org/go/encoding/openapi]
   API generating OpenAPI in Go code.

USING OPENAPI WITH THE CUE COMMAND

The cue command can use OpenAPI constraints in various ways, through its
different subcommands.

Let’s start with a simple CUE schema:

Copied!
schema.cue

Copy code
Copied!

// A schema for the pet API.
package api

$version: "v1.2.3"
// A Pet is a pet that we handle.
#Pet: {
	// A pet has a name.
	name!: string
	// We only handle certain kinds of pets.
	kind!: #Kind
	// Centenarian pets are not handled.
	age?: uint & <100 // TODO: increase limit if the tortoise proposal is accepted.
	...
}

// Kind encodes the different pets we handle.
#Kind: "cat" | "dog" | "goldfish"

The cue def [/docs/reference/command/cue-help-def/] command can
express CUE constraints as OpenAPI data schemas, in OpenAPI’s
components.schemas namespace - but only so long as all the top-level
constraints are specified using CUE definitions.

The OpenAPI info.title field is extracted from the top-level CUE comment
unless the field is specified explicitly. The same goes for OpenAPI’s
info.version field, which is extracted from CUE’s top-level $version field
if info.version isn’t present in the CUE.

Running cue def normalizes our CUE constraints, converting them into the
format specified by the --out parameter.
Be aware of just how long an equivalent OpenAPI definition can become - not
all formats possess CUE’s succinctness and economy of expression!

TERMINAL

Copy code
Copied!

$ cue def schema.cue -o api.pet.yaml --out openapi+yaml

The cue def command creates the api.pet.yaml OpenAPI document:

Copied!
api.pet.yaml

Copy code
Copied!

openapi: 3.0.0
info:
  title: A schema for the pet API.
  version: v1.2.3
paths: {}
components:
  schemas:
    Kind:
      description: Kind encodes the different pets we handle.
      type: string
      enum:
        - cat
        - dog
