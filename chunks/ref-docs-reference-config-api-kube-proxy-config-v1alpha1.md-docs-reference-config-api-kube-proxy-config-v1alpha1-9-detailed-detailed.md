---
doc_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1
chunk_id: ref/docs-reference-config-api-kube-proxy-config-v1alpha1.md/docs-reference-config-api-kube-proxy-config-v1alpha1#9-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 540
summary: ## `ProxyMode` (Alias of `string`) **Appears in:** * [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration) ProxyMode represents modes used by the Kubernetes proxy server....
---

## `ProxyMode`
(Alias of `string`)
**Appears in:**
* [KubeProxyConfiguration](#kubeproxy-config-k8s-io-v1alpha1-KubeProxyConfiguration)
ProxyMode represents modes used by the Kubernetes proxy server.
Three modes of proxy are available on Linux platforms: `iptables`, `ipvs`, and
`nftables`. One mode of proxy is available on Windows platforms: `kernelspace`.
If the proxy mode is unspecified, a default proxy mode will be used (currently this
is `iptables` on Linux and `kernelspace` on Windows). If the selected proxy mode cannot be
used (due to lack of kernel support, missing userspace components, etc) then kube-proxy
will exit with an error.
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
Last modified April 24, 2025 at 8:56 AM PST: [Update config API for v1.33 (2bdd42a2f3)](https://github.com/kubernetes/website/commit/2bdd42a2f3d485f2957f6d3987f44762161f13f6)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [kube-apiserver Configuration (v1beta1)](docs-reference-config-api-apiserver-config-v1beta1.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)