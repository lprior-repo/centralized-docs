---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#14-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's hostnameOverride
token_count: 120
summary: ## Hostname with pod's hostnameOverride FEATURE STATE: `Kubernetes v1.35 [beta]`(enabled by default) Setting a value for `hostnameOverride` in the Pod spec causes the kubelet to unconditionally set...
---

## Hostname with pod's hostnameOverride
FEATURE STATE:
`Kubernetes v1.35 [beta]`(enabled by default)
Setting a value for `hostnameOverride` in the Pod spec causes the kubelet
to unconditionally set both the Pod's hostname and fully qualified domain name (FQDN)
to the `hostnameOverride` value.
The `hostnameOverride` field has a length limitation of 64 characters
and must adhere to the DNS subdomain names standard defined in [RFC 1123](https://datatracker.ietf.org/doc/html/rfc1123).
Example: