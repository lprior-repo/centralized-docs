---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#13-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's setHostnameAsFQDN fields
token_count: 41
summary: you must ensure the combined length of the Pod's `metadata.name` (or `spec.hostname`) and `spec.subdomain` fields results in an FQDN that does not exceed 64 characters.
---

you must ensure the combined length of the Pod's `metadata.name` (or `spec.hostname`)
and `spec.subdomain` fields results in an FQDN that does not exceed 64 characters.