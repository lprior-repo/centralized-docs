---
doc_id: ref/docs-reference-kubernetes-api-common-definitions-quantity.md/docs-reference-kubernetes-api-common-definitions-quantity
chunk_id: ref/docs-reference-kubernetes-api-common-definitions-quantity.md/docs-reference-kubernetes-api-common-definitions-quantity#1-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 66
summary: # Quantity Quantity is a fixed-point representation of a number. `import \"k8s.io/apimachinery/pkg/api/resource\"` Quantity is a fixed-point representation of a number. It provides convenient...
---

# Quantity
Quantity is a fixed-point representation of a number.
`import "k8s.io/apimachinery/pkg/api/resource"`
Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.
The serialization format is: