---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#12-detailed
chunk_level: detailed
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 968
summary: ### kubeadm upgrade plan You can optionally run `kubeadm upgrade plan` before you run `kubeadm upgrade apply`. The `plan` subcommand checks which versions are available to upgrade to and validates...
---

### kubeadm upgrade plan
You can optionally run `kubeadm upgrade plan` before you run `kubeadm upgrade apply`.
The `plan` subcommand checks which versions are available to upgrade
to and validates whether your current cluster is upgradeable.
### kubeadm upgrade diff
This shows what differences would be applied to existing static pod manifests for control plane nodes.
A more verbose way to do the same thing is running `kubeadm upgrade apply --dry-run` or
`kubeadm upgrade node --dry-run`.
### kubeadm upgrade apply
`kubeadm upgrade apply` prepares the cluster for the upgrade of all nodes, and also
upgrades the control plane node where it's run. The steps it performs are:
* Runs preflight checks similarly to `kubeadm init` and `kubeadm join`, ensuring container images are downloaded
and the cluster is in a good state to be upgraded.
* Upgrades the control plane manifest files on disk in `/etc/kubernetes/manifests` and waits
for the kubelet to restart the components if the files have changed.
* Uploads the updated kubeadm and kubelet configurations to the cluster in the `kubeadm-config`
and the `kubelet-config` ConfigMaps (both in the `kube-system` namespace).
* Writes updated kubelet configuration for this node in `/var/lib/kubelet/config.yaml`,
and read the node's `/var/lib/kubelet/instance-config.yaml` file
and patch fields like `containerRuntimeEndpoint`
from this instance configuration into `/var/lib/kubelet/config.yaml`.
* Configures bootstrap token and the `cluster-info` ConfigMap for RBAC rules. This is the same as
in the `kubeadm init` stage and ensures that the cluster continues to support nodes joining with bootstrap tokens.
* Upgrades the kube-proxy and CoreDNS addons conditionally if all existing kube-apiservers in the cluster
have already been upgraded to the target version.
* Performs any post-upgrade tasks, such as, cleaning up deprecated features which are release specific.### kubeadm upgrade node
`kubeadm upgrade node` upgrades a single control plane or worker node after the cluster upgrade has
started (by running `kubeadm upgrade apply`). The command detects if the node is a control plane node by checking
if the file `/etc/kubernetes/manifests/kube-apiserver.yaml` exists. On finding that file, the kubeadm tool
infers that there is a running kube-apiserver Pod on this node.
* Runs preflight checks similarly to `kubeadm upgrade apply`.
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