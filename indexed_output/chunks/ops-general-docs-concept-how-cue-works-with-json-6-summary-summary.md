---
doc_id: ops/general/docs-concept-how-cue-works-with-json
chunk_id: ops/general/docs-concept-how-cue-works-with-json#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary:     \"region\": \"UK\",.     \"cluster\": \"live05\",
---

    "region": "UK",
    "tags": [
        "dev"
    ]
}


Copy code
Copied!

{
    "cluster": "live05",
    "region": "APAC",
    "repository": "source.company.example/alpha"
}

TERMINAL

Copy code
Copied!

$ cue vet -c schema.cue -d '#Config' config-a.json config-b.json config-c.json
region: 2 errors in empty disjunction:
region: conflicting values "APAC" and "UK":
    ./config-b.json:4:15
    ./schema.cue:5:15
    ./schema.cue:9:10
region: conflicting values "IMEA" and "UK":
    ./config-b.json:4:15
    ./schema.cue:5:15
