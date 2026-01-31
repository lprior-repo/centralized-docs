---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#14-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: 		  \"name\": \"John Doe\",. 		  \"iat\": 1516239022
---

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
