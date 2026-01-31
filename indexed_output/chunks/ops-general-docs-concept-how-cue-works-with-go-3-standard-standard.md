---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#3-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 513
summary: 	yamlAsCUE := ctx. BuildFile(yamlFile)
---

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
