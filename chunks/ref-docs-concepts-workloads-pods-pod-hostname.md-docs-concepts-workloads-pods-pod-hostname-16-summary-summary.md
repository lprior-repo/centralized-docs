---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#16-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's hostnameOverride
token_count: 48
summary: #### Note: This only affects the hostname within the Pod; it does not affect the Pod's A or AAAA records in the cluster DNS server. If `hostnameOverride` is set alongside `hostname` and `subdomain`...
---

#### Note:
This only affects the hostname within the Pod; it does not affect the Pod's A or AAAA records in the cluster DNS server.
If `hostnameOverride` is set alongside `hostname` and `subdomain` fields: