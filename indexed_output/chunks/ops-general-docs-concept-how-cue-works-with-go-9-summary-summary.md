---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#9-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: Here, we use load. Instances() to load the package in the current directory:
---


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

