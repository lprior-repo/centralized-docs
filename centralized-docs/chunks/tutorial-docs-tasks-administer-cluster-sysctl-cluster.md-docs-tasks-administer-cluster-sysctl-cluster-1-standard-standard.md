---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 430
summary: # Using sysctls in a Kubernetes Cluster FEATURE STATE: `Kubernetes v1.21 [stable]` This document describes how to configure and use kernel parameters within a Kubernetes cluster using the...
---

# Using sysctls in a Kubernetes Cluster
FEATURE STATE:
`Kubernetes v1.21 [stable]`
This document describes how to configure and use kernel parameters within a
Kubernetes cluster using the [sysctl](/docs/tasks/administer-cluster/sysctl-cluster/)
interface.
#### Note:
Starting from Kubernetes version 1.23, the kubelet supports the use of either `/` or `.`
as separators for sysctl names.
Starting from Kubernetes version 1.25, setting Sysctls for a Pod supports setting sysctls with slashes.
For example, you can represent the same sysctl name as `kernel.shm\_rmid\_forced` using a
period as the separator, or as `kernel/shm\_rmid\_forced` using a slash as a separator.
For more sysctl parameter conversion method details, please refer to
the page [sysctl.d(5)](https://man7.org/linux/man-pages/man5/sysctl.d.5.html) from
the Linux man-pages project.
#### Note:
`sysctl` is a Linux-specific command-line tool used to configure various kernel parameters
and it is not available on non-Linux operating systems.
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
For some steps, you also need to be able to reconfigure the command line
options for the kubelets running on your cluster.