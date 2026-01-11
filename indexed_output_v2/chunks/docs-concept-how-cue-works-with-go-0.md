---
doc_id: docs-concept-how-cue-works-with-go
chunk_id: docs-concept-how-cue-works-with-go#0
chunk_type: table
heading: Introduction
token_count: 3571
summary: # How CUE works with Go | CUE. **Source:** https://cuelang
---

# How CUE works with Go | CUE

**Source:** https://cuelang.org/docs/concept/how-cue-works-with-go/

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


 2. HOW CUE WORKS WITH GO

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
 * go api [/search?q=tag:%22go%20api%22]

CUE is designed to complement and work with the Go programming language.
It offers a powerful API that enables Go code to take advantage of CUE’s
advanced capabilites.
Additionally, CUE makes it easy to use Go as your source of truth by using the
cue command to convert Go types to CUE.

In this guide we’ll demonstrate importing some Kubernetes API code to generate
CUE schemas. We’ll also use the API to convert both CUE and non-CUE data to
native Go values, and validate some Go data natively with CUE.

CONVERTING GO TYPES TO CUE

If you’ve already invested time in developing Go types, you might need them to
be the source of truth in your system whilst also wanting to validate data that
matches those types against the more detailed constraints that CUE allows.

The cue command can help you achieve this as it can convert arbitrary Go types to CUE.
To demonstrate this, we’re going to fetch some Go source code published by the
Kubernetes project, import some types it defines, and use some of the CUE that
gets produced.

Let’s start by downloading a specific version of the k8s.io/api module:

TERMINAL

Copy code
Copied!

$ go get k8s.io/api/apps/v1@v0.29.3
...

We use cue get go to generate CUE definitions from the Go types in the k8s.io/api/apps/v1 package:

TERMINAL

Copy code
Copied!

$ cue get go k8s.io/api/apps/v1

This generates some CUE packages, placing them alongside our main CUE module:

TERMINAL

Copy code
Copied!

$ tree -d cue.mod/gen/k8s.io
cue.mod/gen/k8s.io
|-- api
|   |-- apps
|   |   `-- v1
|   `-- core
|       `-- v1
`-- apimachinery
    `-- pkg
        |-- api
        |   `-- resource
        |-- apis
        |   `-- meta
        |       `-- v1
...

cue get go [/docs/reference/command/cue-help-get-go/]
also has a --local option that generates CUE alongside Go in a main module.

Within our main module, we can import and refer to the CUE definitions generated from the Go types:

Copied!
config.cue

Copy code
Copied!

package config

import (
	core "k8s.io/api/core/v1"
	apps "k8s.io/api/apps/v1"
)

service: [string]:     core.#Service
deployment: [string]:  apps.#Deployment
daemonSet: [string]:   apps.#DaemonSet
statefulSet: [string]: apps.#StatefulSet

Our configuration is currently empty - but any
services, deployments, daemonSets, or statefulSets
that we add will be checked against the schema of the associated Kubernetes type:

TERMINAL

Copy code
Copied!

$ cue eval
service: {}
deployment: {}
daemonSet: {}
statefulSet: {}

A more in-depth example demonstrating how to drive Kubernetes configuration
using CUE can be found in CUE By Example, in
Controlling Kubernetes with CUE [https://github.com/cue-labs/cue-by-example/blob/main/003_kubernetes_tutorial/README.md].

The example above relies on generating CUE within the cue.mod/gen directory
of the CUE module that holds a configuration,
but we are working on a system for providing schemas for well-known services at
a well-known location.
This will remove the need to generate such CUE locally –
see discussion #2939 [/issue/2939] for more details.

USING CUE’S GO API

The Go API injects the power and expressiveness of CUE into your Go programs,
allowing them to
load and validate both CUE and non-CUE data (such as JSON or YAML),
and to
check data marshalled by Go, wherever it comes from.

LOADING CUE DATA

In this example, we load some data from the following CUE file and display it:

Copied!
file.cue

Copy code
Copied!

package example

l: [1, 2, 3]
v: "hello"
message: (v): "world!"

The cuelang.org/go/cue/load package provides a similar interface to the cue
command for loading CUE.

Here, we use load.Instances() to load the package in the current directory:

Copied!
main.go

Copy code
Copied!

package main

import (
	"fmt"

	"cuelang.org/go/cue/cuecontext"
	"cuelang.org/go/cue/load"
)

func main() {
	ctx := cuecontext.New()
	insts := load.Instances([]string{"."}, nil)
	v := ctx.BuildInstance(insts[0])
	fmt.Printf("%v\n", v)
}

Before running, we add a dependency on the cuelang.org/go module and tidy:

TERMINAL

Copy code
Copied!

$ go get cuelang.org/go@v0.15.3
...
$ go mod tidy
...

Finally, running the Go program displays the CUE data:

TERMINAL

Copy code
Copied!

$ go run .
{
	l: [1, 2, 3]
	v: "hello"
	message: {
		hello: "world!"
	}
}

CUE values have a default formatter that renders them sensibly.

LOADING NON-CUE DATA

The API also makes it easy to validate data held in YAML and JSON files.

This example loads a CUE schema that’s embedded in code,
then a YAML data file,
and then validates the data against the schema.

Copied!
main.go

Copy code
Copied!

package main

import (
	"fmt"
	"log"

	"cuelang.org/go/cue"
	"cuelang.org/go/cue/cuecontext"
	"cuelang.org/go/encoding/yaml"
)

const cueSource = `
#Schema: {
	name?: string
	age?:  int
}
`

func main() {
	ctx := cuecontext.New()
	schema := ctx.CompileString(cueSource).LookupPath(cue.ParsePath("#Schema"))

	yamlFile, err := yaml.Extract("data.yml", nil)
	if err != nil {
		log.Fatal(err)
	}

	yamlAsCUE := ctx.BuildFile(yamlFile)

	unified := schema.Unify(yamlAsCUE)
	if err := unified.Validate(); err != nil {
		fmt.Println("❌ YAML: NOT ok")
		log.Fatal(err)
	}

	fmt.Println("✅ YAML: ok")
}

Here’s the data we’ll check against #Schema:

Copied!
data.yml

Copy code
Copied!

name: Charlie Cartwright
age: 99

We finish by adding a dependency on the cuelang.org/go module, tidying,
and running the program:

TERMINAL

Copy code
Copied!

$ go get cuelang.org/go@v0.15.3
...
$ go mod tidy
...
$ go run .
✅ YAML: ok

CHECKING GO DATA WITH CUE SCHEMA

CUE can also validate data that’s only available inside Go.
Perhaps it’s only fetched at runtime, from some file;
or from some remote service over the network.

This time we place our schema in a separate CUE file:

Copied!
schema.cue

Copy code
Copied!

package example

#Person: {
	name?: string
	age?:  int & <=150
}

We embed the schema file using Go embedding [https://pkg.go.dev/embed],
and load it via a string.
Then we use the #Person schema to validate a Go Person,
either logging a fatal error or reporting a successful validation.

Copied!
main.go

Copy code
Copied!

package main

import (
	_ "embed"
	"fmt"
	"log"

	"cuelang.org/go/cue"
	"cuelang.org/go/cue/cuecontext"
)

type Person struct {
	Name string `json:"name"`
	Age  int    `json:"age"`
}

//go:embed schema.cue
var schemaFile string

func main() {
	ctx := cuecontext.New()
	schema := ctx.CompileString(schemaFile).LookupPath(cue.ParsePath("#Person"))

	person := Person{
		Name: "Charlie Cartwright",
		Age:  999,
	}

	personAsCUE := ctx.Encode(person)

	unified := schema.Unify(personAsCUE)
	if err := unified.Validate(); err != nil {
		fmt.Println("❌ Person: NOT ok")
		log.Fatal(err)
	}

	fmt.Println("✅ Person: ok")
}

This time we see that CUE correctly caught a problem in our data:

TERMINAL

Copy code
Copied!

$ go get cuelang.org/go@v0.15.3
...
$ go mod tidy
...
$ go run .
❌ Person: NOT ok
#Person.age: invalid value 999 (out of bound <=150)
exit status 1

FUTURE PLANS

The CUE project believes that its role can be one of interlingua:
a bidirectional bridge between all the formats that CUE speaks,
linking sources of truth with data - wherever they exist.

On the way towards that goal, the project has plans to extend CUE to
directly generate code in Go (and other languages),
beginning with the ability to
declare native types that mirror CUE counterparts.

Looking further forward, the project plans to expand CUE’s generation
capabilities to include
producing native code that implements CUE constraints,
which will enable non-CUE languages to gain highly efficient implementations of
CUE features such as data validation, policy enforcement, and more.

RELATED CONTENT

 * Reference: cue help get go [/docs/reference/command/cue-help-get-go/]
 * Go API:
   cue [https://pkg.go.dev/cuelang.org/go/cue#section-documentation]
   | cue/load [https://pkg.go.dev/cuelang.org/go/cue/load#section-documentation]
   | cue/cuecontext [https://pkg.go.dev/cuelang.org/go/cue/cuecontext#section-documentation]
   | encoding/yaml [https://pkg.go.dev/cuelang.org/go/encoding/yaml#section-documentation]
   | encoding/json [https://pkg.go.dev/cuelang.org/go/encoding/json#section-documentation]
 * CUE By Example:
   Controlling Kubernetes with CUE [https://github.com/cue-labs/cue-by-example/blob/main/003_kubernetes_tutorial/README.md]
 * Discussion #2939 [/issue/2939]: Modules and package management proposal

Last modified December 30, 2025 [https://github.com/cue-lang/cuelang.org/commit/72e9d5a34edd0ed8ba86fe0990b9ef3a945a37fa]

 * encodings [/search?q=tag:encodings]
 * go api [/search?q=tag:%22go%20api%22]


Open share options
 * 
   
   Copy link
   Copied!

 * Share on X (Twitter)
   
   [https://twitter.com/intent/tweet?url=https://cuelang.org/docs/concept/how-cue-works-with-go/&text=CUE%20is%20designed%20to%20complement%20and%20work%20with%20the%20Go%20programming%20language.%20It%20offers%20a%20powerful%20API%20that%20enables%20Go%20code%20to%20take%20advantage%20of%20CUE&rsquo;s%20advanced%20capabilites.%20Additionally,%20CUE%20makes%20it%20easy%20to%20use%20Go%20as%20your%20source%20of%20truth%20by%20using%20the%20cue%20command%20to%20convert%20Go%20types%20to%20CUE.%0aIn%20this%20guide%20we&rsquo;ll%20demonstrate%20importing%20some%20Kubernetes%20API%20code%20to%20generate%20CUE%20schemas.%20We&rsquo;ll%20also%20use%20the%20API%20to%20convert%20both%20CUE%20and%20non-CUE%20data%20to%20native%20Go%20values,%20and%20validate%20some%20Go%20data%20natively%20with%20CUE.%0a]

 * Share on Linkedin
   
   [https://www.linkedin.com/shareArticle?mini=true&url=https://cuelang.org/docs/concept/how-cue-works-with-go/&summary=CUE%20is%20designed%20to%20complement%20and%20work%20with%20the%20Go%20programming%20language.%20It%20offers%20a%20powerful%20API%20that%20enables%20Go%20code%20to%20take%20advantage%20of%20CUE&rsquo;s%20advanced%20capabilites.%20Additionally,%20CUE%20makes%20it%20easy%20to%20use%20Go%20as%20your%20source%20of%20truth%20by%20using%20the%20cue%20command%20to%20convert%20Go%20types%20to%20CUE.%0aIn%20this%20guide%20we&rsquo;ll%20demonstrate%20importing%20some%20Kubernetes%20API%20code%20to%20generate%20CUE%20schemas.%20We&rsquo;ll%20also%20use%20the%20API%20to%20convert%20both%20CUE%20and%20non-CUE%20data%20to%20native%20Go%20values,%20and%20validate%20some%20Go%20data%20natively%20with%20CUE.%0a]


How CUE enables data validation
[/docs/concept/how-cue-enables-data-validation/]How CUE works with JSON
[/docs/concept/how-cue-works-with-json/]
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
   * How CUE works with Go [/docs/concept/how-cue-works-with-go/]
      1. Converting Go types to CUE
      2. Using CUE’s Go API
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
 * Report an Issue [https://github.com/cue-lang/cue/issues/new?labels=Triage,NeedsInvestigation,cuelang.org&title=cuelang.org:%20&template=bug_report.md&body=%23%23%23+What+page+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fcuelang.org%2Fdocs%2Fconcept%2Fhow-cue-works-with-go%2F%0A%0A%23%23%23+What+version+of+the+site+were+you+looking+at%3F%0A%0Ahttps%3A%2F%2Fgithub.com%2Fcue-lang%2Fcuelang.org%2Fcommit%2F6215397cbbdb765fedde5348b4c1f2d7e119dff1%0A%0A%23%23%23+What+did+you+do%3F%0A%0A%0A%0A%23%23%23+What+did+you+expect%3F%0A%0A%0A%0A%23%23%23+What+did+you+see+instead%3F%0A%0A]


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
