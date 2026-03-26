---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#2-standard
chunk_level: standard
chunk_type: prose
heading: Hostname with pod's setHostnameAsFQDN fields
token_count: 389
summary: ## Hostname with pod's setHostnameAsFQDN fields FEATURE STATE: `Kubernetes v1.22 [stable]` When a Pod is configured to have fully qualified domain name (FQDN), its hostname is the short hostname. For...
---

## Hostname with pod's setHostnameAsFQDN fields
FEATURE STATE:
`Kubernetes v1.22 [stable]`
When a Pod is configured to have fully qualified domain name (FQDN), its
hostname is the short hostname. For example, if you have a Pod with the fully
qualified domain name `busybox-1.busybox-subdomain.my-namespace.svc.cluster-domain.example`,
then by default the `hostname` command inside that Pod returns `busybox-1` and the
`hostname --fqdn` command returns the FQDN.
When both `setHostnameAsFQDN: true` and the subdomain field is set in the Pod spec,
the kubelet writes the Pod's FQDN
into the hostname for that Pod's namespace. In this case, both `hostname` and `hostname --fqdn`
return the Pod's FQDN.
The Pod's FQDN is constructed in the same manner as previously defined.
It is composed of the Pod's `spec.hostname` (if specified) or `metadata.name` field,
the `spec.subdomain`, the `namespace` name, and the cluster domain suffix.
#### Note:
In Linux, the hostname field of the kernel (the `nodename` field of `struct utsname`) is limited to 64 characters.
If a Pod enables this feature and its FQDN is longer than 64 character, it will fail to start.
The Pod will remain in `Pending` status (`ContainerCreating` as seen by `kubectl`) generating
error events, such as "Failed to construct FQDN from Pod hostname and cluster domain".
This means that when using this field,
you must ensure the combined length of the Pod's `metadata.name` (or `spec.hostname`)
and `spec.subdomain` fields results in an FQDN that does not exceed 64 characters.