---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#17-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet configuration patterns
token_count: 125
summary: * The path to the DNS resolution file, as specified by the `--resolv-conf` kubelet configuration flag, may differ among operating systems, or depending on whether you are using `systemd-resolved`. If...
---

* The path to the DNS resolution file, as specified by the `--resolv-conf` kubelet
configuration flag, may differ among operating systems, or depending on whether you are using
`systemd-resolved`. If this path is wrong, DNS resolution will fail on the Node whose kubelet
is configured incorrectly.
* The Node API object `.metadata.name` is set to the machine's hostname by default,
unless you are using a cloud provider. You can use the `--hostname-override` flag to override the
default behavior if you need to specify a Node name different from the machine's hostname.