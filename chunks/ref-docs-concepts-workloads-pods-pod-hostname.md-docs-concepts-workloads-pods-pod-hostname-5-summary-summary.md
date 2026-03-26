---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#5-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's hostname and subdomain fields
token_count: 127
summary: The Pod spec includes an optional `hostname` field. When set, this value takes precedence over the Pod's `metadata.name` as the hostname (observed from within the Pod). For example, a Pod with...
---

The Pod spec includes an optional `hostname` field.
When set, this value takes precedence over the Pod's `metadata.name` as the
hostname (observed from within the Pod).
For example, a Pod with spec.hostname set to `my-host` will have its hostname set to `my-host`.
The Pod spec also includes an optional `subdomain` field,
indicating the Pod belongs to a subdomain within its namespace.
If a Pod has `spec.hostname` set to "foo" and spec.subdomain set
to "bar" in the namespace `my-namespace`, its hostname becomes `foo` and its