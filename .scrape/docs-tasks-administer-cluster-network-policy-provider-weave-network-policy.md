---
url: https://kubernetes.io/docs/tasks/administer-cluster/network-policy-provider/weave-network-policy/
title: Weave Net for NetworkPolicy
word_count: 277
filtered: true
elements_removed: 0
density_score: 0.81
---

## Table of Contents

- [Weave Net for NetworkPolicy](#weave-net-for-networkpolicy)
  - [Before you begin](#before-you-begin)
  - [Install the Weave Net addon](#install-the-weave-net-addon)
  - [Test the installation](#test-the-installation)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

# Weave Net for NetworkPolicy
This page shows how to use Weave Net for NetworkPolicy.
## Before you begin
You need to have a Kubernetes cluster. Follow the
[kubeadm getting started guide](/docs/reference/setup-tools/kubeadm/) to bootstrap one.
## Install the Weave Net addon
Follow the [Integrating Kubernetes via the Addon](https://github.com/weaveworks/weave/blob/master/site/kubernetes/kube-addon.md#-installation) guide.
The Weave Net addon for Kubernetes comes with a
[Network Policy Controller](https://github.com/weaveworks/weave/blob/master/site/kubernetes/kube-addon.md#network-policy)
that automatically monitors Kubernetes for any NetworkPolicy annotations on all
namespaces and configures `iptables` rules to allow or block traffic as directed by the policies.
## Test the installation
Verify that the weave works.
Enter the following command:
```
`kubectl get pods -n kube-system -o wide
`
```
The output is similar to this:
```
`NAME READY STATUS RESTARTS AGE IP NODE
weave-net-1t1qg 2/2 Running 0 9d 192.168.2.10 worknode3
weave-net-231d7 2/2 Running 1 7d 10.2.0.17 worknodegpu
weave-net-7nmwt 2/2 Running 3 9d 192.168.2.131 masternode
weave-net-pmw8w 2/2 Running 0 9d 192.168.2.216 worknode2
`
```
Each Node has a weave Pod, and all Pods are `Running` and `2/2 READY`. (`2/2` means that each Pod has `weave` and `weave-npc`.)
## What's next
Once you have installed the Weave Net addon, you can follow the
[Declare Network Policy](/docs/tasks/administer-cluster/declare-network-policy/)
to try out Kubernetes NetworkPolicy. If you have any question, contact us at
[#weave-community on Slack or Weave User Group](https://github.com/weaveworks/weave#getting-help).
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
Last modified February 22, 2024 at 11:32 AM PST: [Update content/en/docs/tasks/administer-cluster/network-policy-provider/weave-network-policy.md (d6459c1823)](https://github.com/kubernetes/website/commit/d6459c1823cb3be4667b34751972fa4be4941184)
## Related Pages

- [Romana for NetworkPolicy](docs-tasks-administer-cluster-network-policy-provider-romana-network-policy.md)
- [Creating a cluster with kubeadm](docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
