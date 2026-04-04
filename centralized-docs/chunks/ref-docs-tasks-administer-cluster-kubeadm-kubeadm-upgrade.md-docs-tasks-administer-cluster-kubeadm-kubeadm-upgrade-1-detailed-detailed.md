---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 991
summary: # Upgrading kubeadm clusters This page explains how to upgrade a Kubernetes cluster created with kubeadm from version 1.34.x to version 1.35.x, and from version 1.35.x to 1.35.y (where `y &gt; x`)....
---

# Upgrading kubeadm clusters
This page explains how to upgrade a Kubernetes cluster created with kubeadm from version
1.34.x to version 1.35.x, and from version
1.35.x to 1.35.y (where `y &gt; x`). Skipping MINOR versions
when upgrading is unsupported. For more details, please visit [Version Skew Policy](/releases/version-skew-policy/).
To see information about upgrading clusters created using older versions of kubeadm,
please refer to following pages instead:
* [Upgrading a kubeadm cluster from 1.33 to 1.34](https://v1-34.docs.kubernetes.io/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/)
* [Upgrading a kubeadm cluster from 1.32 to 1.33](https://v1-33.docs.kubernetes.io/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/)
* [Upgrading a kubeadm cluster from 1.31 to 1.32](https://v1-32.docs.kubernetes.io/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/)
* [Upgrading a kubeadm cluster from 1.30 to 1.31](https://v1-31.docs.kubernetes.io/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/)
The Kubernetes project recommends upgrading to the latest patch releases promptly, and
to ensure that you are running a supported minor release of Kubernetes.
Following this recommendation helps you to stay secure.
The upgrade workflow at high level is the following:
1. Upgrade a primary control plane node.
2. Upgrade additional control plane nodes.
3. Upgrade worker nodes.## Before you begin
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
* `kubeadm upgrade` does not support reconfiguration of an existing cluster. Follow the steps in
[Reconfiguring a kubeadm cluster](/docs/tasks/administer-cluster/kubeadm/kubeadm-reconfigure/) instead.### Considerations when upgrading etcd
Because the `kube-apiserver` static pod is running at all times (even if you
have drained the node), when you perform a kubeadm upgrade which includes an
etcd upgrade, in-flight requests to the server will stall while the new etcd
static pod is restarting. As a workaround, it is possible to actively stop the
`kube-apiserver` process a few seconds before starting the `kubeadm upgrade apply` command. This permits to complete in-flight requests and close existing
connections, and minimizes the consequence of the etcd downtime. This can be
done as follows on control plane nodes: