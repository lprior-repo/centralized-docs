---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#13-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: $ cue export config. cue --out yaml
---

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
