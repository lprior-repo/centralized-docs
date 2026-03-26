---
id: tutorial/docs-tasks-administer-cluster-network-policy-provider-romana-network-policy.md/docs-tasks-administer-cluster-network-policy-provider-romana-network-policy
title: Romana for NetworkPolicy
category: tutorial
tags: ["before", "begin", "contents", "networkpolicy", "romana"]
---

## Table of Contents

* [Romana for NetworkPolicy](#romana-for-networkpolicy)
  * [Before you begin](#before-you-begin)
  * [Installing Romana with kubeadm](#installing-romana-with-kubeadm)
  * [Applying network policies](#applying-network-policies)
  * [Feedback](#feedback)

---

# Romana for NetworkPolicy



 > 
 > **Context**: This page shows how to use Romana for NetworkPolicy.



This page shows how to use Romana for NetworkPolicy.

## Before you begin

Complete steps 1, 2, and 3 of the [kubeadm getting started guide](/docs/reference/setup-tools/kubeadm/).

## Installing Romana with kubeadm

Follow the [containerized installation guide](https://github.com/romana/romana/tree/master/containerize) for kubeadm.

## Applying network policies

To apply network policies use one of the following:

* [Romana network policies](https://github.com/romana/romana/wiki/Romana-policies).
* [Example of Romana network policy](https://github.com/romana/core/blob/master/doc/policy.md).
* The NetworkPolicy API.## What’s next
  Once you have installed Romana, you can follow the
  [Declare Network Policy](/docs/tasks/administer-cluster/declare-network-policy/)
  to try out Kubernetes NetworkPolicy.

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
Last modified January 11, 2023 at 11:12 AM PST: [Update page weights in /tasks/administer-cluster section (b1202c78ff)](https://github.com/kubernetes/website/commit/b1202c78ff58867d67c2fb13f1c13e37d8857a28)

## Related Pages

* [Weave Net for NetworkPolicy](./tutorial-docs-tasks-administer-cluster-network-policy-provider-weave-network-policy.md-docs-tasks-administer-cluster-network-policy-provider-weave-network-policy.md)
* [Creating a cluster with kubeadm](./ref-docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md-docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md)
* [Adding entries to Pod /etc/hosts with HostAliases](./ref-docs-tasks-network-customize-hosts-file-for-pods.md-docs-tasks-network-customize-hosts-file-for-pods.md)
* [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](./tutorial-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
* [Example: Deploying Cassandra with a StatefulSet](./tutorial-docs-tutorials-stateful-application-cassandra.md-docs-tutorials-stateful-application-cassandra.md)
## See Also

- [Documentation Index](./COMPASS.md)
