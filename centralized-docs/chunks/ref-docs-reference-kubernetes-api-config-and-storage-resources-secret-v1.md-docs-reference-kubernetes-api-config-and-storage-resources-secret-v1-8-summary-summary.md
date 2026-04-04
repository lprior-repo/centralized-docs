---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1#8-summary
chunk_level: summary
chunk_type: prose
heading: Secret
token_count: 118
summary: * **immutable** (boolean) Immutable, if set to true, ensures that data stored in the Secret cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at...
---

* **immutable** (boolean)
Immutable, if set to true, ensures that data stored in the Secret cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at any time. Defaulted to nil.
* **stringData** (map[string]string)
stringData allows specifying non-binary secret data in string form. It is provided as a write-only input field for convenience. All keys and values are merged into the data field on write, overwriting any existing values. The stringData field is never output when reading from the API.