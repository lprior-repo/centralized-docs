---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1033
summary: Learn more in the How-to guide Validating JSON using CUE [/docs/howto/validate-json-using-cue/]. PROCESSING AND TRANSFORMING JSON FILES
---


Learn more in the How-to guide Validating JSON using CUE [/docs/howto/validate-json-using-cue/].

PROCESSING AND TRANSFORMING JSON FILES

The cue tool can read and transform JSON files, producing output data in any
shape that’s required. For example:

Copied!
transform.cue

Copy code
Copied!

a: int
b: int
c: 1 + a*b

Copied!
data.json

Copy code
Copied!

{
    "a": 5,
    "b": 4
}

TERMINAL

Copy code
Copied!

$ cue export data.json transform.cue
{
    "a": 5,
    "b": 4,
    "c": 21
}

Learn more about transforming data with CUE in these How-to guides:

 * Transforming JSON with CUE [/docs/howto/transform-json-with-cue/]
 * Combining multiple JSON files into a list [/docs/howto/combine-multiple-json-files-into-a-list/]
 * Combining multiple JSON files by using file metadata [/docs/howto/combine-multiple-json-files-by-using-file-metadata/]

EMBEDDING JSON FILE DATA INSIDE CUE

Requires CUE v0.12.0 or later

The file embedding [/docs/howto/embed-files-in-cue-evaluation/]
feature allows data files (including JSON) to be read when some CUE is evaluated.
This provides an alternative way to use CUE to validate data files against
schemas and constraints, and also gives CUE configurations access to data
stored in non-CUE files:

Copied!
example.cue

Copy code
Copied!

@extern(embed)

package p

_conf: _ @embed(file=config.json)
_data: _ @embed(glob=env/*.json)

info: {
	version: _conf.version
	source:  _data["env/\(_conf.source).json"].text
}

 * 
   
   Copied!
   config.json
 * 
   
   Copied!
   env/foo.json
 * 
   
   Copied!
   env/bar.json

Copy code
Copied!

{
    "version": "1.42.0",
    "source": "bar"
}


Copy code
Copied!

{
    "text": "Some foo"
}


Copy code
Copied!

{
    "text": "A bar"
}

TERMINAL

Copy code
Copied!

$ cue export
{
    "info": {
        "version": "1.42.0",
        "source": "A bar"
    }
}

File embedding is available from CUE v0.12.0 onwards.
Find out more about this powerful validation feature in
Embedding files in a CUE evaluation [/docs/howto/embed-files-in-cue-evaluation/].

ENCODING JSON INSIDE CUE

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
