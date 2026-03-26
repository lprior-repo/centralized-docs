---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#10-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's setHostnameAsFQDN fields
token_count: 40
summary: It is composed of the Pod's `spec.hostname` (if specified) or `metadata.name` field, the `spec.subdomain`, the `namespace` name, and the cluster domain suffix.
---

It is composed of the Pod's `spec.hostname` (if specified) or `metadata.name` field,
the `spec.subdomain`, the `namespace` name, and the cluster domain suffix.