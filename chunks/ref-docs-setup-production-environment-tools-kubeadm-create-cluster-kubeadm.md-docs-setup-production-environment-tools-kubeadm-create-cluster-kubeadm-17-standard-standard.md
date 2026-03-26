---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#17-standard
chunk_level: standard
chunk_type: prose
heading: Version skew policy
token_count: 430
summary: * kubeadm is at 1.35 * `kubernetesVersion` must be at 1.35 or 1.34### kubeadm's skew against the kubelet Similarly to the Kubernetes version, kubeadm can be used with a kubelet version that is the...
---

* kubeadm is at 1.35
* `kubernetesVersion` must be at 1.35 or 1.34### kubeadm's skew against the kubelet
Similarly to the Kubernetes version, kubeadm can be used with a kubelet version that is
the same version as kubeadm or three versions older.
Example:
* kubeadm is at 1.35
* kubelet on the host must be at 1.35, 1.34,
1.33 or 1.32### kubeadm's skew against kubeadm
There are certain limitations on how kubeadm commands can operate on existing nodes or whole clusters
managed by kubeadm.
If new nodes are joined to the cluster, the kubeadm binary used for `kubeadm join` must match
the last version of kubeadm used to either create the cluster with `kubeadm init` or to upgrade
the same node with `kubeadm upgrade`. Similar rules apply to the rest of the kubeadm commands
with the exception of `kubeadm upgrade`.
Example for `kubeadm join`:
* kubeadm version 1.35 was used to create a cluster with `kubeadm init`
* Joining nodes must use a kubeadm binary that is at version 1.35
Nodes that are being upgraded must use a version of kubeadm that is the same MINOR
version or one MINOR version newer than the version of kubeadm used for managing the
node.
Example for `kubeadm upgrade`:
* kubeadm version 1.34 was used to create or upgrade the node
* The version of kubeadm used for upgrading the node must be at 1.34
or 1.35
To learn more about the version skew between the different Kubernetes component see
the [Version Skew Policy](/releases/version-skew-policy/).