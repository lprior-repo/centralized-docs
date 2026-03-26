---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#13-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the kubelet cgroup driver
token_count: 84
summary: and passing it to the local node kubelet. On each node, kubeadm detects the CRI socket and stores its details into the `/var/lib/kubelet/instance-config.yaml` file. When executing the `init`, `join`,...
---

and passing it to the local node kubelet.
On each node, kubeadm detects the CRI socket and stores its details into the `/var/lib/kubelet/instance-config.yaml` file.
When executing the `init`, `join`, or `upgrade` subcommands,
kubeadm patches the `containerRuntimeEndpoint` value from this instance configuration into `/var/lib/kubelet/config.yaml`.