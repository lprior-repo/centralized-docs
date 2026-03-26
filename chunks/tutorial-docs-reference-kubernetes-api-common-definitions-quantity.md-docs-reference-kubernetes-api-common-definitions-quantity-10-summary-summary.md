---
doc_id: tutorial/docs-reference-kubernetes-api-common-definitions-quantity.md/docs-reference-kubernetes-api-common-definitions-quantity
chunk_id: tutorial/docs-reference-kubernetes-api-common-definitions-quantity.md/docs-reference-kubernetes-api-common-definitions-quantity#10-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: * 1.5 will be serialized as \"1500m\" - 1.5Gi will be serialized as \"1536Mi\" Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this...
---

* 1.5 will be serialized as "1500m" - 1.5Gi will be serialized as "1536Mi"
Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.
Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)
This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.