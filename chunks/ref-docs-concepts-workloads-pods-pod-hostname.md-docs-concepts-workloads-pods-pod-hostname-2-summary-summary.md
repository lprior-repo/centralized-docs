---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#2-summary
chunk_level: summary
chunk_type: prose
heading: Default Pod hostname
token_count: 113
summary: ## Default Pod hostname When a Pod is created, its hostname (as observed from within the Pod) is derived from the Pod's metadata.name value. Both the hostname and its corresponding fully qualified...
---

## Default Pod hostname
When a Pod is created, its hostname (as observed from within the Pod)
is derived from the Pod's metadata.name value.
Both the hostname and its corresponding fully qualified domain name (FQDN)
are set to the metadata.name value (from the Pod's perspective)
```
`apiVersion: v1
kind: Pod
metadata:
name: busybox-1
spec:
containers:
- image: busybox:1.28
command:
- sleep
- "3600"
name: busybox
`
```