---
url: https://kubernetes.io/docs/tasks/administer-cluster/kubeadm/upgrading-windows-nodes/
title: Upgrading Windows nodes
word_count: 422
filtered: true
elements_removed: 0
density_score: 0.92
---

## Table of Contents

- [Upgrading Windows nodes](#upgrading-windows-nodes)
  - [Before you begin](#before-you-begin)
    - [Upgrade kubeadm](#upgrade-kubeadm)
    - [Drain the node](#drain-the-node)
    - [Upgrade the kubelet configuration](#upgrade-the-kubelet-configuration)
    - [Upgrade kubelet and kube-proxy](#upgrade-kubelet-and-kube-proxy)
      - [Note:](#note)
    - [Uncordon the node](#uncordon-the-node)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

# Upgrading Windows nodes
FEATURE STATE:
`Kubernetes v1.18 [beta]`
This page explains how to upgrade a Windows node created with kubeadm.
## Before you begin
You need to have shell access to all the nodes, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial
on a cluster with at least two nodes that are not acting as control plane hosts.
Your Kubernetes server must be at or later than version 1.17.
To check the version, enter `kubectl version`.
* Familiarize yourself with [the process for upgrading the rest of your kubeadm
cluster](/docs/tasks/administer-cluster/kubeadm/kubeadm-upgrade/). You will want to
upgrade the control plane nodes before upgrading your Windows nodes.## Upgrading worker nodes
### Upgrade kubeadm
1. From the Windows node, upgrade kubeadm:
```
`# replace 1.35.0 with your desired version
curl.exe -Lo &lt;path-to-kubeadm.exe&gt; "https://dl.k8s.io/v1.35.0/bin/windows/amd64/kubeadm.exe"
`
```
### Drain the node
1. From a machine with access to the Kubernetes API,
prepare the node for maintenance by marking it unschedulable and evicting the workloads:
```
`# replace &lt;node-to-drain&gt; with the name of your node you are draining
kubectl drain &lt;node-to-drain&gt; --ignore-daemonsets
`
```
You should see output similar to this:
```
`node/ip-172-31-85-18 cordoned
node/ip-172-31-85-18 drained
`
```
### Upgrade the kubelet configuration
1. From the Windows node, call the following command to sync new kubelet configuration:
```
`kubeadm upgrade node
`
```
### Upgrade kubelet and kube-proxy
1. From the Windows node, upgrade and restart the kubelet:
```
`stop-service kubelet
curl.exe -Lo &lt;path-to-kubelet.exe&gt; "https://dl.k8s.io/v1.35.0/bin/windows/amd64/kubelet.exe"
restart-service kubelet
`
```
2. From the Windows node, upgrade and restart the kube-proxy.
```
`stop-service kube-proxy
curl.exe -Lo &lt;path-to-kube-proxy.exe&gt; "https://dl.k8s.io/v1.35.0/bin/windows/amd64/kube-proxy.exe"
restart-service kube-proxy
`
```
#### Note:
If you are running kube-proxy in a HostProcess container within a Pod, and not as a Windows Service,
you can upgrade kube-proxy by applying a newer version of your kube-proxy manifests.
### Uncordon the node
1. From a machine with access to the Kubernetes API,
bring the node back online by marking it schedulable:
```
`# replace &lt;node-to-drain&gt; with the name of your node
kubectl uncordon &lt;node-to-drain&gt;
`
```
## What's next
* See how to [Upgrade Linux nodes](/docs/tasks/administer-cluster/kubeadm/upgrading-linux-nodes/).
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified September 13, 2024 at 4:59 PM PST: [Reorganize kubeadm node tasks (c8bb00db5d)](https://github.com/kubernetes/website/commit/c8bb00db5deb264d9ae42247c121e18021f53f09)
## Related Pages

- [Creating a cluster with kubeadm](docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md)
- [Creating Highly Available Clusters with kubeadm](docs-setup-production-environment-tools-kubeadm-high-availability.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
