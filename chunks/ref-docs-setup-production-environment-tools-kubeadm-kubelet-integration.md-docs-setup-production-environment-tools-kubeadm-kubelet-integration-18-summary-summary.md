---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#18-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet configuration patterns
token_count: 119
summary: * Currently, the kubelet cannot automatically detect the cgroup driver used by the container runtime, but the value of `--cgroup-driver` must match the cgroup driver used by the container runtime to...
---

* Currently, the kubelet cannot automatically detect the cgroup driver used by the container runtime,
but the value of `--cgroup-driver` must match the cgroup driver used by the container runtime to ensure
the health of the kubelet.
* To specify the container runtime you must set its endpoint with the
`--container-runtime-endpoint=&lt;path&gt;` flag.
The recommended way of applying such instance-specific configuration is by using
[`KubeletConfiguration` patches](/docs/setup/production-environment/tools/kubeadm/control-plane-flags/#patches).