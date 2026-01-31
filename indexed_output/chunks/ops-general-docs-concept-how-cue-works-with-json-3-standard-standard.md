---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#3-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 515
summary: CUE is frequently used to generate configuration files.  Some systems allow
---


CUE is frequently used to generate configuration files. Some systems allow
their configuration files to contain JSON encoded in string fields,
irrespective of the file’s main data format.

CUE’s standard library provides
a built-in json package [https://pkg.go.dev/cuelang.org/go/pkg/encoding/json]
containing functions that generate, parse, validate, and format JSON from
within CUE - some of which are shown here.

GENERATING ENCODED JSON

In this example a Kubernetes ConfigMap contains a JSON file encoded as a
single string field, in a YAML document. This is enabled by the
json.Marshal function:

Copied!
config.cue

Copy code
Copied!

import "encoding/json"

configMap: data: "point.json": json.Marshal({
	x: 1.2
	y: 3.45
})

TERMINAL

Copy code
Copied!

$ cue export config.cue --out yaml
configMap:
  data:
    point.json: '{"x":1.2,"y":3.45}'

PARSING ENCODED JSON

The json.Unmarshal function performs the reverse operation to json.Marshal:
it turns a string containing JSON into the structure represented by
the encoded data.

Here, some encoded JSON data (a JSON Web Token) is emitted as YAML:

Copied!
token.cue

Copy code
Copied!

import "encoding/json"

_jwt: {
	header: #"{"alg":"HS256","typ":"JWT"}"#
	payload: """
		{
		  "sub": "1234567890",
		  "name": "John Doe",
		  "iat": 1516239022
		}
		"""
}
output: header:  json.Unmarshal(_jwt.header)
output: payload: json.Unmarshal(_jwt.payload)

TERMINAL

Copy code
Copied!

$ cue export token.cue --out yaml
output:
  header:
    alg: HS256
    typ: JWT
  payload:
    sub: "1234567890"
    name: John Doe
    iat: 1516239022

VALIDATING ENCODED JSON

The json.Validate function allows encoded JSON to be validated against
native CUE schema constraints.

Here, each member of the item map is checked against the #Dimensions
schema. The cue tool correctly catches and flags up two problems with the
data:

Copied!
furniture.cue

Copy code
Copied!

import "encoding/json"

#Dimensions: {
	width:  number
	depth:  number
	height: number
}

// Validate each member of the map against a schema.
