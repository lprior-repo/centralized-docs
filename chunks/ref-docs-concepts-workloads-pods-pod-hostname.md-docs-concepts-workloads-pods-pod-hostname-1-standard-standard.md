---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#1-standard
chunk_level: standard
chunk_type: prose
heading: Hostname with pod's hostname and subdomain fields
token_count: 401
summary: # Pod Hostname This page explains how to set a Pod's hostname, potential side effects after configuration, and the underlying mechanics. ## Default Pod hostname When a Pod is created, its hostname...
---

# Pod Hostname
This page explains how to set a Pod's hostname,
potential side effects after configuration, and the underlying mechanics.
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
The Pod created by this manifest will have its hostname and fully qualified domain name (FQDN) set to `busybox-1`.
## Hostname with pod's hostname and subdomain fields
The Pod spec includes an optional `hostname` field.
When set, this value takes precedence over the Pod's `metadata.name` as the
hostname (observed from within the Pod).
For example, a Pod with spec.hostname set to `my-host` will have its hostname set to `my-host`.
The Pod spec also includes an optional `subdomain` field,
indicating the Pod belongs to a subdomain within its namespace.
If a Pod has `spec.hostname` set to "foo" and spec.subdomain set
to "bar" in the namespace `my-namespace`, its hostname becomes `foo` and its
fully qualified domain name (FQDN) becomes
`foo.bar.my-namespace.svc.cluster-domain.example` (observed from within the Pod).
When both hostname and subdomain are set, the cluster's DNS server will
create A and/or AAAA records based on these fields.
Refer to: [Pod's hostname and subdomain fields](/docs/concepts/services-networking/dns-pod-service/#pod-hostname-and-subdomain-field).