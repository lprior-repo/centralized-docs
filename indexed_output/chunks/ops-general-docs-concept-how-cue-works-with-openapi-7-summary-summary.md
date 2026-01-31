---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#7-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: The cue def command creates the api. yaml OpenAPI document:
---


The cue def command creates the api.pet.yaml OpenAPI document:

Copied!
api.pet.yaml

Copy code
Copied!

openapi: 3.0.0
info:
  title: A schema for the pet API.
  version: v1.2.3
paths: {}
components:
  schemas:
    Kind:
      description: Kind encodes the different pets we handle.
      type: string
      enum:
        - cat
        - dog
        - goldfish
    Pet:
      description: A Pet is a pet that we handle.
      type: object
      required:
        - name
        - kind
      properties:
        name:
