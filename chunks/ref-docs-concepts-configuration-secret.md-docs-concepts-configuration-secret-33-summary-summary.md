---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#33-summary
chunk_level: summary
chunk_type: table
heading: Types of Secret
token_count: 125
summary: When creating a Secret, you can specify its type using the `type` field of the [Secret](/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/) resource, or certain equivalent...
---

When creating a Secret, you can specify its type using the `type` field of
the [Secret](/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/)
resource, or certain equivalent `kubectl` command line flags (if available).
The Secret type is used to facilitate programmatic handling of the Secret data.
Kubernetes provides several built-in types for some common usage scenarios.
These types vary in terms of the validations performed and the constraints
Kubernetes imposes on them.
|Built-in Type|Usage|
|`Opaque`|arbitrary user-defined data|
|`kubernetes.io/service-account-token`