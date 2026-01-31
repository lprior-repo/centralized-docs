---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#12-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: kind: conflicting values \"dog\" and \"tortoise\":. kind: conflicting values \"goldfish\" and \"tortoise\":
---

    ./api.pet.yaml:26:11
    ./jonathan.yml:2:7
kind: conflicting values "dog" and "tortoise":
    ./api.pet.yaml:26:11
    ./jonathan.yml:2:7
kind: conflicting values "goldfish" and "tortoise":
    ./api.pet.yaml:26:11
    ./jonathan.yml:2:7

Perhaps our #Pet schema should be updated to handle more animal types!

USING OPENAPI WITH THE GO API

CUE can also access and generate OpenAPI through its
encoding/openapi [https://pkg.go.dev/cuelang.org/go/encoding/openapi]
Go API.

Generating an OpenAPI definition can be as simple as this:
