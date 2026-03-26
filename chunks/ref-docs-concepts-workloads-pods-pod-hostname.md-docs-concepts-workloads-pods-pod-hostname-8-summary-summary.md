---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#8-summary
chunk_level: summary
chunk_type: prose
heading: Hostname with pod's setHostnameAsFQDN fields
token_count: 118
summary: FEATURE STATE: `Kubernetes v1.22 [stable]` When a Pod is configured to have fully qualified domain name (FQDN), its hostname is the short hostname. For example, if you have a Pod with the fully...
---

FEATURE STATE:
`Kubernetes v1.22 [stable]`
When a Pod is configured to have fully qualified domain name (FQDN), its
hostname is the short hostname. For example, if you have a Pod with the fully
qualified domain name `busybox-1.busybox-subdomain.my-namespace.svc.cluster-domain.example`,
then by default the `hostname` command inside that Pod returns `busybox-1` and the
`hostname --fqdn` command returns the FQDN.
When both `setHostnameAsFQDN: true`