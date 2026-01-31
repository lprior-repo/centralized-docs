---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 514
summary: a well-known location. This will remove the need to generate such CUE locally –
---

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
