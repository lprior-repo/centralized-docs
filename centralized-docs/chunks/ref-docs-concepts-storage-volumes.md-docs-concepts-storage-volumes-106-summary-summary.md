---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#106-summary
chunk_level: summary
chunk_type: prose
heading: Using subPath
token_count: 118
summary: ### Using subPath with expanded environment variables FEATURE STATE: `Kubernetes v1.17 [stable]` Use the `subPathExpr` field to construct `subPath` directory names from downward API environment...
---

### Using subPath with expanded environment variables
FEATURE STATE:
`Kubernetes v1.17 [stable]`
Use the `subPathExpr` field to construct `subPath` directory names from
downward API environment variables.
The `subPath` and `subPathExpr` properties are mutually exclusive.
In this example, a `Pod` uses `subPathExpr` to create a directory `pod1` within
the `hostPath` volume `/var/log/pods`.
The `hostPath` volume takes the `Pod` name from the `downwardAPI`.