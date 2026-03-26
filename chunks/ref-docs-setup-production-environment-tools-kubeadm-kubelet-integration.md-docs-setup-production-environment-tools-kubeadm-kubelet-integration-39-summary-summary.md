---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#39-summary
chunk_level: summary
chunk_type: prose
heading: The kubelet drop-in file for systemd
token_count: 116
summary: * The KubeConfig file to use for the TLS Bootstrap is `/etc/kubernetes/bootstrap-kubelet.conf`, but it is only used if `/etc/kubernetes/kubelet.conf` does not exist. * The KubeConfig file with the...
---

* The KubeConfig file to use for the TLS Bootstrap is `/etc/kubernetes/bootstrap-kubelet.conf`,
but it is only used if `/etc/kubernetes/kubelet.conf` does not exist.
* The KubeConfig file with the unique kubelet identity is `/etc/kubernetes/kubelet.conf`.
* The file containing the kubelet's ComponentConfig is `/var/lib/kubelet/config.yaml`.
* The dynamic environment file that contains `KUBELET\_KUBEADM\_ARGS` is sourced from `/var/lib/kubelet/kubeadm-flags.env`.