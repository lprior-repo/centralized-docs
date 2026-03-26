---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#9-standard
chunk_level: standard
chunk_type: table
heading: The kubelet drop-in file for systemd
token_count: 404
summary: This file specifies the default locations for all of the files managed by kubeadm for the kubelet. * The KubeConfig file to use for the TLS Bootstrap is `/etc/kubernetes/bootstrap-kubelet.conf`, but...
---

This file specifies the default locations for all of the files managed by kubeadm for the kubelet.
* The KubeConfig file to use for the TLS Bootstrap is `/etc/kubernetes/bootstrap-kubelet.conf`,
but it is only used if `/etc/kubernetes/kubelet.conf` does not exist.
* The KubeConfig file with the unique kubelet identity is `/etc/kubernetes/kubelet.conf`.
* The file containing the kubelet's ComponentConfig is `/var/lib/kubelet/config.yaml`.
* The dynamic environment file that contains `KUBELET\_KUBEADM\_ARGS` is sourced from `/var/lib/kubelet/kubeadm-flags.env`.
* The file that can contain user-specified flag overrides with `KUBELET\_EXTRA\_ARGS` is sourced from
`/etc/default/kubelet` (for DEBs), or `/etc/sysconfig/kubelet` (for RPMs). `KUBELET\_EXTRA\_ARGS`
is last in the flag chain and has the highest priority in the event of conflicting settings.## Kubernetes binaries and package contents
The DEB and RPM packages shipped with the Kubernetes releases are:
|Package name|Description|
|`kubeadm`|Installs the `/usr/bin/kubeadm` CLI tool and the [kubelet drop-in file](#the-kubelet-drop-in-file-for-systemd) for the kubelet.|
|`kubelet`|Installs the `/usr/bin/kubelet` binary.|
|`kubectl`|Installs the `/usr/bin/kubectl` binary.|
|`cri-tools`|Installs the `/usr/bin/crictl` binary from the [cri-tools git repository](https://github.com/kubernetes-sigs/cri-tools).|
|`kubernetes-cni`|Installs the `/opt/cni/bin` binaries from the [plugins git repository](https://github.com/containernetworking/plugins).|