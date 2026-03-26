---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#95-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 122
summary: Secrets can be mounted as data volumes or exposed as [environment variables](/docs/concepts/containers/container-environment/) to be used by a container in a Pod. Secrets can also be used by other...
---

Secrets can be mounted as data volumes or exposed as
[environment variables](/docs/concepts/containers/container-environment/)
to be used by a container in a Pod. Secrets can also be used by other parts of the
system, without being directly exposed to the Pod. For example, Secrets can hold
credentials that other parts of the system should use to interact with external
systems on your behalf.
Secret volume sources are validated to ensure that the specified object
reference actually points to an object of type Secret. Therefore, a Secret
needs to be created before any Pods that depend on it.