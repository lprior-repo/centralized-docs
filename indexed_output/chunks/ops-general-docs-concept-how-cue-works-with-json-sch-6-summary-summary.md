---
doc_id: ops/general/docs-concept-how-cue-works-with-json-sch
chunk_id: ops/general/docs-concept-how-cue-works-with-json-sch#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary:             \"default\": null.         \"home phone\": {
---

            },
            "default": null
        },
        "home phone": {
            "type": "string",
            "deprecated": true
        }
    }
}

We use cue import to convert the JSON Schema to CUE:

TERMINAL

Copy code
Copied!

$ cue import -l '#Person:' schema.json

cue import recognises JSON Schema from its signature fields, and uses the
schema’s constraints to create a shorter, more readable CUE representation.
Our -l parameter tells cue to place the constraints inside the #Person
definition:
