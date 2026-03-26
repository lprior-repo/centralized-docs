---
doc_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname
chunk_id: ref/docs-concepts-workloads-pods-pod-hostname.md/docs-concepts-workloads-pods-pod-hostname#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 744
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified August 07, 2025 at 8:18 AM PST: [Follow up PR 51414 comments (1e994aaed3)](https://github.com/kubernetes/website/commit/1e994aaed3a15cc3a3240879ace29dee10f97c0e)
## Related Pages

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)