---
doc_id: ops/general/docs-concept-how-cue-works-with-yaml
chunk_id: ops/general/docs-concept-how-cue-works-with-yaml#6-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: region: 2 errors in empty disjunction:. region: conflicting values \"APAC\" and \"UK\":
---

region: 2 errors in empty disjunction:
region: conflicting values "APAC" and "UK":
    ./config-b.yaml:3:9
    ./schema.cue:5:15
    ./schema.cue:9:10
region: conflicting values "IMEA" and "UK":
    ./config-b.yaml:3:9
    ./schema.cue:5:15
    ./schema.cue:9:19
cluster: invalid value "live03333333333333" (does not satisfy strings.MaxRunes(16)):
    ./schema.cue:4:15
    ./config-b.yaml:1:10
    ./schema.cue:4:32
repository: invalid value "github.com/Alex_Personal_Account/alpha-fork" (out of bound =~"^source\\.company\\.example/"):
