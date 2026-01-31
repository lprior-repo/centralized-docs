---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: Println(\"✅ YAML: ok\"). Here’s the data we’ll check against #Schema:
---

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
