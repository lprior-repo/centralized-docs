---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Using the `cgroupfs` driver
token_count: 818
summary: # Configuring a cgroup driver This page explains how to configure the kubelet's cgroup driver to match the container runtime cgroup driver for kubeadm clusters. ## Before you begin You should be...
---

# Configuring a cgroup driver
This page explains how to configure the kubelet's cgroup driver to match the container
runtime cgroup driver for kubeadm clusters.
## Before you begin
You should be familiar with the Kubernetes
[container runtime requirements](/docs/setup/production-environment/container-runtimes/).
## Configuring the container runtime cgroup driver
The [Container runtimes](/docs/setup/production-environment/container-runtimes/) page
explains that the `systemd` driver is recommended for kubeadm based setups instead
of the kubelet's [default](/docs/reference/config-api/kubelet-config.v1beta1/) `cgroupfs` driver,
because kubeadm manages the kubelet as a
[systemd service](/docs/setup/production-environment/tools/kubeadm/kubelet-integration/).
The page also provides details on how to set up a number of different container runtimes with the
`systemd` driver by default.
## Configuring the kubelet cgroup driver
kubeadm allows you to pass a `KubeletConfiguration` structure during `kubeadm init`.
This `KubeletConfiguration` can include the `cgroupDriver` field which controls the cgroup
driver of the kubelet.
#### Note:
In v1.22 and later, if the user does not set the `cgroupDriver` field under `KubeletConfiguration`,
kubeadm defaults it to `systemd`.
In Kubernetes v1.28, you can enable automatic detection of the
cgroup driver as an alpha feature.
See [systemd cgroup driver](/docs/setup/production-environment/container-runtimes/#systemd-cgroup-driver)
for more details.
A minimal example of configuring the field explicitly:
```
`# kubeadm-config.yaml
kind: ClusterConfiguration
apiVersion: kubeadm.k8s.io/v1beta4
kubernetesVersion: v1.21.0
---
kind: KubeletConfiguration
apiVersion: kubelet.config.k8s.io/v1beta1
cgroupDriver: systemd
`
```
Such a configuration file can then be passed to the kubeadm command:
```
`kubeadm init --config kubeadm-config.yaml
`
```
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
See the below section on "[Modify the kubelet ConfigMap](#modify-the-kubelet-configmap)" for details on
how to be explicit about the value.
If you wish to configure a container runtime to use the `cgroupfs` driver,
you must refer to the documentation of the container runtime of your choice.