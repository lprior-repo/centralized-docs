---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#28-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm upgrade workflow internal design
token_count: 494
summary: * Runs preflight checks similarly to `kubeadm init` and `kubeadm join`, ensuring container images are downloaded and the cluster is in a good state to be upgraded. * Upgrades the control plane...
---

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