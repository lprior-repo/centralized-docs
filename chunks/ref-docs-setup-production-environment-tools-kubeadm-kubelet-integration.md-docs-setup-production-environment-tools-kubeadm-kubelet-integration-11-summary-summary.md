---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#11-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet configuration patterns
token_count: 105
summary: ### Propagating cluster-level configuration to each kubelet You can provide the kubelet with default values to be used by `kubeadm init` and `kubeadm join` commands. Interesting examples include...
---

### Propagating cluster-level configuration to each kubelet
You can provide the kubelet with default values to be used by `kubeadm init` and `kubeadm join`
commands. Interesting examples include using a different container runtime or setting the default subnet
used by services.
If you want your services to use the subnet `10.96.0.0/12` as the default for services, you can pass
the `--service-cidr` parameter to kubeadm: