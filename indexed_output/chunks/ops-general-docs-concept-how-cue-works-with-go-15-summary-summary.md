---
doc_id: ops/general/docs-concept-how-cue-works-with-go
chunk_id: ops/general/docs-concept-how-cue-works-with-go#15-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 143
summary: Println(\"✅ Person: ok\"). This time we see that CUE correctly caught a problem in our data:
---

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
