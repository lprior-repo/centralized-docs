---
doc_id: ops/general/docs-concept-how-cue-works-with-openapi
chunk_id: ops/general/docs-concept-how-cue-works-with-openapi#8-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary:           description: A pet has a name.           type: string
---

          description: A pet has a name.
          type: string
        kind:
          $ref: '#/components/schemas/Kind'
        age:
          description: Centenarian pets are not handled.
          type: integer
          minimum: 0
          maximum: 100
          exclusiveMaximum: true

Because CUE is more expressive than OpenAPI,
it isn’t possible to generate a precise OpenAPI equivalent for every CUE constraint.
CUE does the best conversion it can,
limited by what OpenAPI’s data schemas can represent.
