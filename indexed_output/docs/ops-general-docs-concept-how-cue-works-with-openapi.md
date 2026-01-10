---
id: ops/general/docs-concept-how-cue-works-with-openapi
title: Docs Concept How Cue Works With Openapi
category: ops
tags: ["openapi", "ops", "works"]
---

# Docs Concept How Cue Works With Openapi

> **Context**: **Source:** https://cuelang.org/docs/concept/how-cue-works-with-openapi/

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
“Structural OpenAPI” form required by CRDs targeting Kubernetes version 1.15
and later.

FUTURE PLANS

One of CUE’s goals is to act as an interlingua: a bidirectional bridge
between all the formats that CUE speaks, linking constraints with data sources
of truth, no matter where they exist.

For now, only OpenAPI’s data schemas, in the components.schemas namespace,
are handled by CUE. More complete support is tracked in issue #3133 [/issue/3133].

RELATED CONTENT

 * Reference: cue help def [/docs/reference/command/cue-help-def/]
 * Reference: cue help import [/docs/reference/command/cue-help-import/]
 * Reference: cue help vet [/docs/reference/command/cue-help-vet/]
 * The encoding/openapi [https://pkg.go.dev/cuelang.org/go/encoding/openapi] package
 * Issue #3133 [/issue/3133] tracks the support of other namespaces defined by
   the OpenAPI standard [https://github.com/OAI/OpenAPI-Specification/tree/3.0.0]

Last modified November 6, 2025 [https://github.com/cue-lang/cuelang.org/commit/c4d2e727a59f5158a2be4946992d96c4612a9b88]

 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-openapi/&text=CUE%20has%20first%20class%20support%20for%20OpenAPI%20data%20schemas:%20the%20cue%20command%20automatically%20recognises%20OpenAPI%20by%20its%20signature%20fields,%20and%20the%20Go%20API%20has%20packages%20dedicated%20to%20the%20format.%20Specifically,%20CUE%20supports%20the%20OpenAPI%203.0.0%20standard%20through%20its%20components.schemas%20namespace%20for%20data%20schemas.%0aConstraints%20stored%20as%20OpenAPI%20data%20schemas%20are%20available%20for%20cue%20commands%20to%20use%20as%20if%20they%20were%20expressed%20in%20CUE.%20This%20allows%20you%20to%20work%20with%20these%20constraints%20directly,%20using%20them%20to%20validate%20data,%20and%20to%20represent%20them%20natively%20in%20CUE%e2%80%99s%20significantly%20more%20concise%20form.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-openapi/&summary=CUE%20has%20first%20class%20support%20for%20OpenAPI%20data%20schemas:%20the%20cue%20command%20automatically%20recognises%20OpenAPI%20by%20its%20signature%20fields,%20and%20the%20Go%20API%20has%20packages%20dedicated%20to%20the%20format.%20Specifically,%20CUE%20supports%20the%20OpenAPI%203.0.0%20standard%20through%20its%20components.schemas%20namespace%20for%20data%20schemas.%0aConstraints%20stored%20as%20OpenAPI%20data%20schemas%20are%20available%20for%20cue%20commands%20to%20use%20as%20if%20they%20were%20expressed%20in%20CUE.%20This%20allows%20you%20to%20work%20with%20these%20constraints%20directly,%20using%20them%20to%20validate%20data,%20and%20to%20represent%20them%20natively%20in%20CUE%e2%80%99s%20significantly%20more%20concise%20form.%0a]


How CUE works with JSON Schema
[/docs/concept/how-cue-works-with-json-schema/]How CUE works with Protocol Buffers
[/docs/concept/how-cue-works-with-protocol-buffers/]
 * Introduction [/docs/introduction/]
 * Tour [/docs/tour/]
 * Integrations [/docs/integration/]
 * Tutorials [/docs/tutorial/]
 * How-to Guides [/docs/howto/]
 * Concept Guides [/docs/concept/]
   * Popular guides [/docs/concept/popular-guides/]
   * The Logic of CUE [/docs/concept/the-logic-of-cue/]
   * Modules [/docs/concept/modules/]
   * Frequently Asked Questions [/docs/concept/faq/]
   * How CUE works with OpenAPI [/docs/concept/how-cue-works-with-openapi/]
      1. Using OpenAPI with the cue command
      2. Using OpenAPI with the Go API
      3. Future plans
      4. Related content
 * References [/docs/reference/]

Hide side navigation


Show side navigation

Get Started

 * Documentation [/docs/]
 * Language Tour [/docs/tour/]
 * Playground [/play/]
 * Install CUE [/docs/introduction/installation/]

Community

 * The CUE Community [/community]
 * Contributing [https://github.com/cue-lang/cue/blob/master/CONTRIBUTING.md#contribution-guide]
 * Code of Conduct [/docs/reference/code-of-conduct/]
 * Slack Workspace [/s/slack]
 * Discord Server [/s/discord]

Connect

 * GitHub [https://github.com/cue-lang/cue]
 * X (Twitter) [https://twitter.com/cue_lang]
 * Bluesky [https://bsky.app/profile/cuelang.org]
 * YouTube [https://www.youtube.com/@cuelang/videos]

 * © 2025 CUE
 * Privacy policy [/privacy-policy/]
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fconcept%2Fhow-cue-works-with-openapi%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


Homepage of CUE [/]
CUE v0.15 is now available – learn more about its new features and improvements [https://github.com/cue-lang/cue/releases/tag/v0.15.0]
Install CUE

[/docs/introduction/installation/]

Close

Homepage of CUE [/]


Hide menu
 * Documentation [/docs/]
 * Play [/play/]
 * Community [/community/]
 * Install [/docs/introduction/installation/]
 * 

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

## See Also

- [Documentation Index](./COMPASS.md)

