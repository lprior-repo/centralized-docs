---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#3-standard
chunk_level: standard
chunk_type: prose
heading: Kubelet configuration patterns
token_count: 284
summary: ### Providing instance-specific configuration details Some hosts require specific kubelet configurations due to differences in hardware, operating system, networking, or other host-specific...
---

### Providing instance-specific configuration details
Some hosts require specific kubelet configurations due to differences in hardware, operating system,
networking, or other host-specific parameters. The following list provides a few examples.
* The path to the DNS resolution file, as specified by the `--resolv-conf` kubelet
configuration flag, may differ among operating systems, or depending on whether you are using
`systemd-resolved`. If this path is wrong, DNS resolution will fail on the Node whose kubelet
is configured incorrectly.
* The Node API object `.metadata.name` is set to the machine's hostname by default,
unless you are using a cloud provider. You can use the `--hostname-override` flag to override the
default behavior if you need to specify a Node name different from the machine's hostname.
* Currently, the kubelet cannot automatically detect the cgroup driver used by the container runtime,
but the value of `--cgroup-driver` must match the cgroup driver used by the container runtime to ensure
the health of the kubelet.
* To specify the container runtime you must set its endpoint with the
`--container-runtime-endpoint=&lt;path&gt;` flag.
The recommended way of applying such instance-specific configuration is by using
[`KubeletConfiguration` patches](/docs/setup/production-environment/tools/kubeadm/control-plane-flags/#patches).