---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-kubelet-integration.md/docs-setup-production-environment-tools-kubeadm-kubelet-integration#3-detailed
chunk_level: detailed
chunk_type: table
heading: The kubelet drop-in file for systemd
token_count: 949
summary: ## The kubelet drop-in file for systemd `kubeadm` ships with configuration for how systemd should run the kubelet. Note that the kubeadm CLI command never touches this drop-in file. This...
---

## The kubelet drop-in file for systemd
`kubeadm` ships with configuration for how systemd should run the kubelet.
Note that the kubeadm CLI command never touches this drop-in file.
This configuration file installed by the `kubeadm`
[package](https://github.com/kubernetes/release/blob/cd53840/cmd/krel/templates/latest/kubeadm/10-kubeadm.conf) is written to
`/usr/lib/systemd/system/kubelet.service.d/10-kubeadm.conf` and is used by systemd.
It augments the basic
[`kubelet.service`](https://github.com/kubernetes/release/blob/cd53840/cmd/krel/templates/latest/kubelet/kubelet.service).
If you want to override that further, you can make a directory `/etc/systemd/system/kubelet.service.d/`
(not `/usr/lib/systemd/system/kubelet.service.d/`) and put your own customizations into a file there.
For example, you might add a new local file `/etc/systemd/system/kubelet.service.d/local-overrides.conf`
to override the unit settings configured by `kubeadm`.
Here is what you are likely to find in `/usr/lib/systemd/system/kubelet.service.d/10-kubeadm.conf`:
#### Note:
The contents below are just an example. If you don't want to use a package manager
follow the guide outlined in the ([Without a package manager](/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#k8s-install-2))
section.
```
`[Service]
Environment="KUBELET\_KUBECONFIG\_ARGS=--bootstrap-kubeconfig=/etc/kubernetes/bootstrap-kubelet.conf --kubeconfig=/etc/kubernetes/kubelet.conf"
Environment="KUBELET\_CONFIG\_ARGS=--config=/var/lib/kubelet/config.yaml"
# This is a file that "kubeadm init" and "kubeadm join" generate at runtime, populating
# This is a file that the user can use for overrides of the kubelet args as a last resort. Preferably,
# the user should use the .NodeRegistration.KubeletExtraArgs object in the configuration files instead.
# KUBELET\_EXTRA\_ARGS should be sourced from this file.
EnvironmentFile=-/etc/default/kubelet
ExecStart=
ExecStart=/usr/bin/kubelet $KUBELET\_KUBECONFIG\_ARGS $KUBELET\_CONFIG\_ARGS $KUBELET\_KUBEADM\_ARGS $KUBELET\_EXTRA\_ARGS
`
```
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