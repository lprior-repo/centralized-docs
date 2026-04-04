---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#29-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 410
summary: * For control plane nodes, upgrades the control plane manifest files on disk in `/etc/kubernetes/manifests` and waits for the kubelet to restart the components if the files have changed. * Writes...
---

* For control plane nodes, upgrades the control plane manifest files on disk in `/etc/kubernetes/manifests`
and waits for the kubelet to restart the components if the files have changed.
* Writes updated kubelet configuration for this node in `/var/lib/kubelet/config.yaml`,
and read the node's `/var/lib/kubelet/instance-config.yaml` file and
patch fields like `containerRuntimeEndpoint`
from this instance configuration into `/var/lib/kubelet/config.yaml`.
* (For control plane nodes) upgrades the kube-proxy and CoreDNS
[addons](/docs/concepts/cluster-administration/addons/) conditionally, provided that all existing
API servers in the cluster have already been upgraded to the target version.
* Performs any post-upgrade tasks, such as cleaning up deprecated features which are release specific.## kubeadm reset workflow internal design
You can use the `kubeadm reset` subcommand on a node where kubeadm commands previously executed.
This subcommand performs a **best-effort** cleanup of the node.
If certain actions fail you must intervene and perform manual cleanup.
The command supports phases.
See [`kubeadm reset phase`](/docs/reference/setup-tools/kubeadm/kubeadm-reset-phase/) for more details.
The command supports a configuration file.
Additionally:
* IPVS, iptables and nftables rules are **not** cleaned up.
* CNI (network plugin) configuration is **not** cleaned up.
* `.kube/` in the user's home directory is **not** cleaned up.
The command has the following stages:
* Runs preflight checks on the node to determine if its healthy.
* For control plane nodes, removes any local etcd member data.
* Stops the kubelet.
* Stops running containers.
* Unmounts any mounted directories in `/var/lib/kubelet`.
* Deletes any files and directories managed by kubeadm in `/var/lib/kubelet` and `/etc/kubernetes`.