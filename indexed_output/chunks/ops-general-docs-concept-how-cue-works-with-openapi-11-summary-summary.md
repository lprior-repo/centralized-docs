---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#11-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 132
summary: name: Jonathan. kind: tortoise
---


Copied!
jonathan.yml

Copy code
Copied!

name: Jonathan
kind: tortoise
website: https://en.wikipedia.org/wiki/Jonathan_(tortoise)

Let’s validate Jonathan against the contents of our OpenAPI document, api.pet.yaml.
CUE automatically recognises the constraints in the components.schemas namespace,
making them available as the #Pet schema:

TERMINAL

Copy code
Copied!

$ cue vet -c api.pet.yaml jonathan.yml -d '#Pet'
kind: 3 errors in empty disjunction:
kind: conflicting values "cat" and "tortoise":
    ./api.pet.yaml:11:7
