---
id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
title: Configuring a cgroup driver
category: ref
tags: ["before", "begin", "cgroup", "configuring", "contents"]
---

## Table of Contents

* [Configuring a cgroup driver](#configuring-a-cgroup-driver)
  * [Before you begin](#before-you-begin)
  * [Configuring the container runtime cgroup driver](#configuring-the-container-runtime-cgroup-driver)
  * [Configuring the kubelet cgroup driver](#configuring-the-kubelet-cgroup-driver)
    * [Note:](#note)
    * [Note:](#note)
  * [Using the `cgroupfs` driver](#using-the-cgroupfs-driver)
  * [Migrating to the `systemd` driver](#migrating-to-the-systemd-driver)
    * [Note:](#note)
    * [Modify the kubelet ConfigMap](#modify-the-kubelet-configmap)
    * [Update the cgroup driver on all nodes](#update-the-cgroup-driver-on-all-nodes)
  * [Feedback](#feedback)

---

# Configuring a cgroup driver



 > 
 > **Context**: This page explains how to configure the kubelet's cgroup driver to match the container runtime cgroup driver for kubeadm clusters.



This page explains how to configure the kubelet’s cgroup driver to match the container
runtime cgroup driver for kubeadm clusters.

## Before you begin

You should be familiar with the Kubernetes
[container runtime requirements](/docs/setup/production-environment/container-runtimes/).

## Configuring the container runtime cgroup driver

The [Container runtimes](/docs/setup/production-environment/container-runtimes/) page
explains that the `systemd` driver is recommended for kubeadm based setups instead
of the kubelet’s [default](/docs/reference/config-api/kubelet-config.v1beta1/) `cgroupfs` driver,
because kubeadm manages the kubelet as a
[systemd service](/docs/setup/production-environment/tools/kubeadm/kubelet-integration/).
The page also provides details on how to set up a number of different container runtimes with the
`systemd` driver by default.

## Configuring the kubelet cgroup driver

kubeadm allows you to pass a `KubeletConfiguration` structure during `kubeadm init`.
This `KubeletConfiguration` can include the `cgroupDriver` field which controls the cgroup
driver of the kubelet.

### Note:

In v1.22 and later, if the user does not set the `cgroupDriver` field under `KubeletConfiguration`,
kubeadm defaults it to `systemd`.
In Kubernetes v1.28, you can enable automatic detection of the
cgroup driver as an alpha feature.
See [systemd cgroup driver](/docs/setup/production-environment/container-runtimes/#systemd-cgroup-driver)
for more details.
A minimal example of configuring the field explicitly:

````
`# kubeadm-config.yaml
kind: ClusterConfiguration
apiVersion: kubeadm.k8s.io/v1beta4
kubernetesVersion: v1.21.0
---
kind: KubeletConfiguration
apiVersion: kubelet.config.k8s.io/v1beta1
cgroupDriver: systemd
`
````

Such a configuration file can then be passed to the kubeadm command:

````
`kubeadm init --config kubeadm-config.yaml
`
````

#### Note:

Kubeadm uses the same `KubeletConfiguration` for all nodes in the cluster.
The `KubeletConfiguration` is stored in a [ConfigMap](/docs/concepts/configuration/configmap/)
object under the `kube-system` namespace.
Executing the sub commands `init`, `join` and `upgrade` would result in kubeadm
writing the `KubeletConfiguration` as a file under `/var/lib/kubelet/config.yaml`
and passing it to the local node kubelet.
On each node, kubeadm detects the CRI socket and stores its details into the `/var/lib/kubelet/instance-config.yaml` file.
When executing the `init`, `join`, or `upgrade` subcommands,
kubeadm patches the `containerRuntimeEndpoint` value from this instance configuration into `/var/lib/kubelet/config.yaml`.

## Using the `cgroupfs` driver

To use `cgroupfs` and to prevent `kubeadm upgrade` from modifying the
`KubeletConfiguration` cgroup driver on existing setups, you must be explicit
about its value. This applies to a case where you do not wish future versions
of kubeadm to apply the `systemd` driver by default.
See the below section on “[Modify the kubelet ConfigMap](#modify-the-kubelet-configmap)” for details on
how to be explicit about the value.
If you wish to configure a container runtime to use the `cgroupfs` driver,
you must refer to the documentation of the container runtime of your choice.

## Migrating to the `systemd` driver

To change the cgroup driver of an existing kubeadm cluster from `cgroupfs` to `systemd` in-place,
a similar procedure to a kubelet upgrade is required. This must include both
steps outlined below.

### Note:

Alternatively, it is possible to replace the old nodes in the cluster with new ones
that use the `systemd` driver. This requires executing only the first step below
before joining the new nodes and ensuring the workloads can safely move to the new
nodes before deleting the old nodes.

### Modify the kubelet ConfigMap

* Call `kubectl edit cm kubelet-config -n kube-system`.
* Either modify the existing `cgroupDriver` value or add a new field that looks like this:

````
`cgroupDriver: systemd
`
````

This field must be present under the `kubelet:` section of the ConfigMap.

### Update the cgroup driver on all nodes

For each node in the cluster:

* [Drain the node](/docs/tasks/administer-cluster/safely-drain-node/) using `kubectl drain &lt;node-name&gt; --ignore-daemonsets`
* Stop the kubelet using `systemctl stop kubelet`
* Stop the container runtime
* Modify the container runtime cgroup driver to `systemd`
* Set `cgroupDriver: systemd` in `/var/lib/kubelet/config.yaml`
* Start the container runtime
* Start the kubelet using `systemctl start kubelet`
* [Uncordon the node](/docs/tasks/administer-cluster/safely-drain-node/) using `kubectl uncordon &lt;node-name&gt;`
  Execute these steps on nodes one at a time to ensure workloads
  have sufficient time to schedule on different nodes.
  Once the process is complete ensure that all nodes and workloads are healthy.

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
Last modified July 04, 2025 at 11:28 PM PST: [Add documentation for graduating the NodeLocalCRISocket to Beta (0135b1b08b)](https://github.com/kubernetes/website/commit/0135b1b08b57e526723e9a6981b5a9de518c3ea4)

## Related Pages

* [Secrets](./ref-docs-concepts-configuration-secret.md-docs-concepts-configuration-secret.md)
* [Debugging DNS Resolution](./tutorial-docs-tasks-administer-cluster-dns-debugging-resolution.md-docs-tasks-administer-cluster-dns-debugging-resolution.md)
* [Configuring each kubelet in your cluster using kubeadm](./ref-docs-setup-production-environment-tools-kubeadm-kubelet-integration.md-docs-setup-production-environment-tools-kubeadm-kubelet-integration.md)
* [Reserve Compute Resources for System Daemons](./tutorial-docs-tasks-administer-cluster-reserve-compute-resources.md-docs-tasks-administer-cluster-reserve-compute-resources.md)
* [Switching from Polling to CRI Event-based Updates to Container Status](./ref-docs-tasks-administer-cluster-switch-to-evented-pleg.md-docs-tasks-administer-cluster-switch-to-evented-pleg.md)
## See Also

- [Documentation Index](./COMPASS.md)
