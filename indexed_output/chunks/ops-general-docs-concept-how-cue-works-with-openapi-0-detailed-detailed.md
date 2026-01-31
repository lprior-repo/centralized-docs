---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#0-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1027
summary: # How CUE works with OpenAPI | CUE. **Source:** https://cuelang
---

# How CUE works with OpenAPI | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-openapi/

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


 2. HOW CUE WORKS WITH OPENAPI

jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews
jpluscplusm [https://github.com/jpluscplusm.png]
Jonathan Matthews

Github profile

[https://github.com/jpluscplusm]

Search all content by this author

[/search/?q=author:jpluscplusm]
myitcv [https://github.com/myitcv.png]
Paul Jolly
myitcv [https://github.com/myitcv.png]
Paul Jolly

Github profile

[https://github.com/myitcv]

Search all content by this author

[/search/?q=author:myitcv]
 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]

CUE has first class support for OpenAPI data schemas: the cue command
automatically recognises OpenAPI by its signature fields, and the Go API has
packages dedicated to the format.
Specifically, CUE supports the
OpenAPI 3.0.0 standard [https://github.com/OAI/OpenAPI-Specification/tree/3.0.0]
through its components.schemas namespace for data schemas.

Constraints stored as OpenAPI data schemas are available for cue commands to
use as if they were expressed in CUE. This allows you to work with these
constraints directly, using them to validate data, and to represent them
natively in CUE’s significantly more concise form.

In this guide, we’ll see:

 * cue def [/docs/reference/command/cue-help-def/]
   generating an OpenAPI data schema from a CUE definition,
 * cue import [/docs/reference/command/cue-help-import/]
   turning the generated OpenAPI back into CUE,
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
