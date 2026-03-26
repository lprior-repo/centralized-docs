---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#3-standard
chunk_level: standard
chunk_type: prose
heading: Hostname with pod's hostnameOverride
token_count: 416
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
```
`apiVersion: v1
kind: Pod
metadata:
name: busybox-2-busybox-example-domain
spec:
hostnameOverride: busybox-2.busybox.example.domain
containers:
- image: busybox:1.28
command:
- sleep
- "3600"
name: busybox
`
```
#### Note:
This only affects the hostname within the Pod; it does not affect the Pod's A or AAAA records in the cluster DNS server.
If `hostnameOverride` is set alongside `hostname` and `subdomain` fields:
* The hostname inside the Pod is overridden to the `hostnameOverride` value.
* The Pod's A and/or AAAA records in the cluster DNS server are still generated based on the `hostname` and `subdomain` fields.
Note: If `hostnameOverride` is set, you cannot simultaneously set the `hostNetwork` and `setHostnameAsFQDN` fields.
The API server will explicitly reject any create request attempting this combination.
For details on behavior when `hostnameOverride` is set in combination with
other fields (hostname, subdomain, setHostnameAsFQDN, hostNetwork),
see the table in the [KEP-4762 design details](https://github.com/kubernetes/enhancements/blob/master/keps/sig-network/4762-allow-arbitrary-fqdn-as-pod-hostname/README.md#design-details).