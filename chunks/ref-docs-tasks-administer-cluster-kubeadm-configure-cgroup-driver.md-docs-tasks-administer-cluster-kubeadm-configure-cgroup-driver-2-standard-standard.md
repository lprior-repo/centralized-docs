---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#2-standard
chunk_level: standard
chunk_type: prose
heading: Configuring the kubelet cgroup driver
token_count: 459
summary: ## Configuring the kubelet cgroup driver kubeadm allows you to pass a `KubeletConfiguration` structure during `kubeadm init`. This `KubeletConfiguration` can include the `cgroupDriver` field which...
---

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