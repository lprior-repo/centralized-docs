---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 420
summary: * Make sure you read the [release notes](https://git.k8s.io/kubernetes/CHANGELOG) carefully. * The cluster should use a static control plane and etcd pods or external etcd. * Make sure to back up any...
---

* Make sure you read the [release notes](https://git.k8s.io/kubernetes/CHANGELOG) carefully.
* The cluster should use a static control plane and etcd pods or external etcd.
* Make sure to back up any important components, such as app-level state stored in a database.
`kubeadm upgrade` does not touch your workloads, only components internal to Kubernetes, but backups are always a best practice.
* [Swap must be disabled](https://serverfault.com/questions/684771/best-way-to-disable-swap-in-linux).### Additional information
* The instructions below outline when to drain each node during the upgrade process.
If you are performing a **minor** version upgrade for any kubelet, you **must**
first drain the node (or nodes) that you are upgrading. In the case of control plane nodes,
they could be running CoreDNS Pods or other critical workloads. For more information see
[Draining nodes](/docs/tasks/administer-cluster/safely-drain-node/).
* The Kubernetes project recommends that you match your kubelet and kubeadm versions.
You can instead use a version of kubelet that is older than kubeadm, provided it is within the
range of supported versions.
For more details, please visit [kubeadm's skew against the kubelet](/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/#kubeadm-s-skew-against-the-kubelet).
* All containers are restarted after upgrade, because the container spec hash value is changed.
* To verify that the kubelet service has successfully restarted after the kubelet has been upgraded,
you can execute `systemctl status kubelet` or view the service logs with `journalctl -xeu kubelet`.
* `kubeadm upgrade` supports `--config` with a
[`UpgradeConfiguration` API type](/docs/reference/config-api/kubeadm-config.v1beta4/) which can
be used to configure the upgrade process.