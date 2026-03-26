---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#14-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet configuration patterns
token_count: 101
summary: [`KubeletConfiguration`](/docs/reference/config-api/kubelet-config.v1beta1/). The `KubeletConfiguration` allows the user to specify flags such as the cluster DNS IP addresses expressed as a list of...
---

[`KubeletConfiguration`](/docs/reference/config-api/kubelet-config.v1beta1/).
The `KubeletConfiguration` allows the user to specify flags such as the cluster DNS IP addresses expressed as
a list of values to a camelCased key, illustrated by the following example:
```
`apiVersion: kubelet.config.k8s.io/v1beta1
kind: KubeletConfiguration
clusterDNS:
- 10.96.0.10
`
```