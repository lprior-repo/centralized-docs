---
doc_id: docs-concept-how-cue-works-with-json-schema
chunk_id: docs-concept-how-cue-works-with-json-schema#0
heading: Introduction
token_count: 3394
summary: # How CUE works with JSON Schema | CUE. **Source:** https://cuelang
---

# How CUE works with JSON Schema | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-json-schema/

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


 2. HOW CUE WORKS WITH JSON SCHEMA

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
 * go api [/search?q=tag:%22go%20api%22]

CUE has first class support for JSON Schema [https://json-schema.org/]:
both the cue command and the Go API understand the format.

Constraints stored as JSON Schema are available for cue commands to use as if
they were expressed in CUE.
This allows you to work with JSON Schema constraints directly, using them to
validate data, and to represent them natively in CUE’s more succinct and
expressive form.

In this guide we’ll see:

 * cue import [/docs/reference/command/cue-help-import/] converting a
   JSON Schema to CUE,
 * cue vet [/docs/reference/command/cue-help-vet/] using JSON Schema
   constraints directly,
 * and the
   encoding/jsonschema [https://pkg.go.dev/cuelang.org/go/encoding/jsonschema]
   Go API validating data against a JSON Schema.

The ability to export CUE constraints as JSON Schema is tracked in issue #929 [/issue/929].

USING JSON SCHEMA WITH THE CUE COMMAND

The cue import [/docs/reference/command/cue-help-import/] command can
produce CUE from JSON Schema.

Let’s start with this JSON Schema:

Copied!
schema.json

Copy code
Copied!

{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "title": "Main Person schema.",
    "description": "This schema defines a person.",
    "required": [
        "name"
    ],
    "properties": {
        "name": {
            "description": "What is this person called?",
            "type": "string",
            "minLength": 1
        },
        "address": {
            "description": "Where does this person live?",
            "type": "string",
            "minLength": 1,
            "maxLength": 200
        },
        "children": {
            "description": "This is a very long comment for some reason, which will keep going and going past the point where it should probably have stopped.",
            "type": "array",
            "items": {
                "type": "string"
            },
            "default": null
        },
        "home phone": {
            "type": "string",
            "deprecated": true
        }
    }
}

We use cue import to convert the JSON Schema to CUE:

TERMINAL

Copy code
Copied!

$ cue import -l '#Person:' schema.json

cue import recognises JSON Schema from its signature fields, and uses the
schema’s constraints to create a shorter, more readable CUE representation.
Our -l parameter tells cue to place the constraints inside the #Person
definition:

Copied!
schema.cue

Copy code
Copied!

// Main Person schema.
//
// This schema defines a person.

import "strings"

#Person: {
	@jsonschema(schema="https://json-schema.org/draft/2020-12/schema")

	// What is this person called?
	name!: strings.MinRunes(1)

	// Where does this person live?
	address?: strings.MinRunes(1) & strings.MaxRunes(200)

	// This is a very long comment for some reason, which will keep
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
	address: "Ripon, North Yorkshire"
}
$ go run . schema.json bad.json
name: conflicting values ["Charlie","Cartwright"] and strings.MinRunes(1) (mismatched types list and string):
    bad.json:2:13
    schema.json:13:13
exit status 1

FUTURE PLANS

One of CUE’s goals is to act as an interlingua: a bidirectional bridge
between all the formats that CUE speaks, linking constraints and data sources
of truth, no matter where they exist.

To meet this goal, CUE will gain the ability to export native CUE constraints
as JSON Schema, enabling their use by tools that aren’t aware of CUE. This is
tracked in issue #929 [/issue/929].

RELATED CONTENT

 * Reference: cue help import [/docs/reference/command/cue-help-import/]
 * The encoding/jsonschema [https://pkg.go.dev/cuelang.org/go/encoding/jsonschema] Go API
 * Reference: cue help vet [/docs/reference/command/cue-help-vet/]
 * Reference: cue help filetypes [/docs/reference/command/cue-help-filetypes/]
 * Issue #929 [/issue/929] tracks the conversion of CUE to JSON Schema

Last modified September 4, 2025 [https://github.com/cue-lang/cuelang.org/commit/c675de963f4124145b48e2681dab7b4aacab71e2]

 * encodings [/search?q=tag:encodings]
 * cue command [/search?q=tag:%22cue%20command%22]
 * go api [/search?q=tag:%22go%20api%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-json-schema/&text=CUE%20has%20first%20class%20support%20for%20JSON%20Schema:%20both%20the%20cue%20command%20and%20the%20Go%20API%20understand%20the%20format.%0aConstraints%20stored%20as%20JSON%20Schema%20are%20available%20for%20cue%20commands%20to%20use%20as%20if%20they%20were%20expressed%20in%20CUE.%20This%20allows%20you%20to%20work%20with%20JSON%20Schema%20constraints%20directly,%20using%20them%20to%20validate%20data,%20and%20to%20represent%20them%20natively%20in%20CUE&rsquo;s%20more%20succinct%20and%20expressive%20form.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-json-schema/&summary=CUE%20has%20first%20class%20support%20for%20JSON%20Schema:%20both%20the%20cue%20command%20and%20the%20Go%20API%20understand%20the%20format.%0aConstraints%20stored%20as%20JSON%20Schema%20are%20available%20for%20cue%20commands%20to%20use%20as%20if%20they%20were%20expressed%20in%20CUE.%20This%20allows%20you%20to%20work%20with%20JSON%20Schema%20constraints%20directly,%20using%20them%20to%20validate%20data,%20and%20to%20represent%20them%20natively%20in%20CUE&rsquo;s%20more%20succinct%20and%20expressive%20form.%0a]


How CUE works with JSON
[/docs/concept/how-cue-works-with-json/]How CUE works with OpenAPI
[/docs/concept/how-cue-works-with-openapi/]
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
   * How CUE works with JSON Schema [/docs/concept/how-cue-works-with-json-schema/]
      1. Using JSON Schema with the cue command
      2. Using JSON Schema with the Go API
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fconcept%2Fhow-cue-works-with-json-schema%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
