---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#12-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's setHostnameAsFQDN fields
token_count: 128
summary: In Linux, the hostname field of the kernel (the `nodename` field of `struct utsname`) is limited to 64 characters. If a Pod enables this feature and its FQDN is longer than 64 character, it will fail...
---

In Linux, the hostname field of the kernel (the `nodename` field of `struct utsname`) is limited to 64 characters.
If a Pod enables this feature and its FQDN is longer than 64 character, it will fail to start.
The Pod will remain in `Pending` status (`ContainerCreating` as seen by `kubectl`) generating
error events, such as "Failed to construct FQDN from Pod hostname and cluster domain".
This means that when using this field,
you must ensure the combined length of the Pod's `metadata.name` (or `spec.hostname`)
and `spec.subdomain`