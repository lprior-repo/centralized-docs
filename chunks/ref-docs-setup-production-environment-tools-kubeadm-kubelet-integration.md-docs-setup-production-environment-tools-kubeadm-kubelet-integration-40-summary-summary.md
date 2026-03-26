---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#40-summary
chunk_level: summary
chunk_type: table
heading: The kubelet drop-in file for systemd
token_count: 126
summary: * The file that can contain user-specified flag overrides with `KUBELET\_EXTRA\_ARGS` is sourced from `/etc/default/kubelet` (for DEBs), or `/etc/sysconfig/kubelet` (for RPMs). `KUBELET\_EXTRA\_ARGS`...
---

* The file that can contain user-specified flag overrides with `KUBELET\_EXTRA\_ARGS` is sourced from
`/etc/default/kubelet` (for DEBs), or `/etc/sysconfig/kubelet` (for RPMs). `KUBELET\_EXTRA\_ARGS`
is last in the flag chain and has the highest priority in the event of conflicting settings.## Kubernetes binaries and package contents
The DEB and RPM packages shipped with the Kubernetes releases are:
|Package name|Description|
|`kubeadm`|Installs the `/usr/bin/kubeadm`