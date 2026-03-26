---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#8-detailed
chunk_level: detailed
chunk_type: prose
heading: Troubleshooting
token_count: 945
summary: ## Version skew policy While kubeadm allows version skew against some components that it manages, it is recommended that you match the kubeadm version with the versions of the control plane...
---

## Version skew policy
While kubeadm allows version skew against some components that it manages, it is recommended that you
match the kubeadm version with the versions of the control plane components, kube-proxy and kubelet.
### kubeadm's skew against the Kubernetes version
kubeadm can be used with Kubernetes components that are the same version as kubeadm
or one version older. The Kubernetes version can be specified to kubeadm by using the
`--kubernetes-version` flag of `kubeadm init` or the
[`ClusterConfiguration.kubernetesVersion`](/docs/reference/config-api/kubeadm-config.v1beta4/)
field when using `--config`. This option will control the versions
of kube-apiserver, kube-controller-manager, kube-scheduler and kube-proxy.
Example:
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
### Cluster resilience
The cluster created here has a single control-plane node, with a single etcd database
running on it. This means that if the control-plane node fails, your cluster may lose
data and may need to be recreated from scratch.
Workarounds:
* Regularly [back up etcd](https://etcd.io/docs/v3.5/op-guide/recovery/). The
etcd data directory configured by kubeadm is at `/var/lib/etcd` on the control-plane node.
* Use multiple control-plane nodes. You can read
[Options for Highly Available topology](/docs/setup/production-environment/tools/kubeadm/ha-topology/) to pick a cluster
topology that provides [high-availability](/docs/setup/production-environment/tools/kubeadm/high-availability/).
### Platform compatibility
kubeadm deb/rpm packages and binaries are built for amd64, arm (32-bit), arm64, ppc64le, and s390x
following the [multi-platform proposal](https://git.k8s.io/design-proposals-archive/multi-platform.md).
Multiplatform container images for the control plane and addons are also supported since v1.12.
Only some of the network providers offer solutions for all platforms. Please consult the list of
network providers above or the documentation from each provider to figure out whether the provider
supports your chosen platform.
## Troubleshooting
If you are running into difficulties with kubeadm, please consult our
[troubleshooting docs](/docs/setup/production-environment/tools/kubeadm/troubleshooting-kubeadm/).