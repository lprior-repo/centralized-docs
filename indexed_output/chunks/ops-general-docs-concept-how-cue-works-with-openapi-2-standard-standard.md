---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#2-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 517
summary:       description: A Pet is a pet that we handle.       type: object
---

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
