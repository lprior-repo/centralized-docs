---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#6-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's hostname and subdomain fields
token_count: 122
summary: \"foo\" and spec.subdomain set to \"bar\" in the namespace `my-namespace`, its hostname becomes `foo` and its fully qualified domain name (FQDN) becomes `foo.bar.my-namespace.svc.cluster-domain.example`...
---

"foo" and spec.subdomain set
to "bar" in the namespace `my-namespace`, its hostname becomes `foo` and its
fully qualified domain name (FQDN) becomes
`foo.bar.my-namespace.svc.cluster-domain.example` (observed from within the Pod).
When both hostname and subdomain are set, the cluster's DNS server will
create A and/or AAAA records based on these fields.
Refer to: [Pod's hostname and subdomain fields](/docs/concepts/services-networking/dns-pod-service/#pod-hostname-and-subdomain-field).