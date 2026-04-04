---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1#7-summary
chunk_level: summary
chunk_type: prose
heading: Secret
token_count: 90
summary: * **data** (map[string][]byte) Data contains the secret data. Each key must consist of alphanumeric characters, '-', '\_' or '.'. The serialized form of the secret data is a base64 encoded string,...
---

* **data** (map[string][]byte)
Data contains the secret data. Each key must consist of alphanumeric characters, '-', '\_' or '.'. The serialized form of the secret data is a base64 encoded string, representing the arbitrary (possibly non-string) data value here. Described in [https://tools.ietf.org/html/rfc4648#section-4](https://tools.ietf.org/html/rfc4648#section-4)