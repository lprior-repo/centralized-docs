---
doc_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1
chunk_id: ref/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1.md/docs-reference-kubernetes-api-config-and-storage-resources-secret-v1#1-standard
chunk_level: standard
chunk_type: prose
heading: Secret
token_count: 418
summary: # Secret Secret holds secret data of a certain type. `apiVersion: v1` `import \"k8s.io/api/core/v1\"` ## Secret Secret holds secret data of a certain type. The total bytes of the values in the Data...
---

# Secret
Secret holds secret data of a certain type.
`apiVersion: v1`
`import "k8s.io/api/core/v1"`
## Secret
Secret holds secret data of a certain type. The total bytes of the values in the Data field must be less than MaxSecretSize bytes.
* **apiVersion**: v1
* **kind**: Secret
* **metadata** ([ObjectMeta](https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#ObjectMeta))
Standard object's metadata. More info: [https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata](https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata)
* **data** (map[string][]byte)
Data contains the secret data. Each key must consist of alphanumeric characters, '-', '\_' or '.'. The serialized form of the secret data is a base64 encoded string, representing the arbitrary (possibly non-string) data value here. Described in [https://tools.ietf.org/html/rfc4648#section-4](https://tools.ietf.org/html/rfc4648#section-4)
* **immutable** (boolean)
Immutable, if set to true, ensures that data stored in the Secret cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at any time. Defaulted to nil.
* **stringData** (map[string]string)
stringData allows specifying non-binary secret data in string form. It is provided as a write-only input field for convenience. All keys and values are merged into the data field on write, overwriting any existing values. The stringData field is never output when reading from the API.
* **type** (string)
Used to facilitate programmatic handling of secret data. More info: [https://kubernetes.io/docs/concepts/configuration/secret/#secret-types](https://kubernetes.io/docs/concepts/configuration/secret/#secret-types)