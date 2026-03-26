---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#20-summary
chunk_level: summary
chunk_type: prose
heading: Migrating to the `systemd` driver
token_count: 107
summary: * [Drain the node](/docs/tasks/administer-cluster/safely-drain-node/) using `kubectl drain &lt;node-name&gt; --ignore-daemonsets` * Stop the kubelet using `systemctl stop kubelet` * Stop the...
---

* [Drain the node](/docs/tasks/administer-cluster/safely-drain-node/) using `kubectl drain &lt;node-name&gt; --ignore-daemonsets`
* Stop the kubelet using `systemctl stop kubelet`
* Stop the container runtime
* Modify the container runtime cgroup driver to `systemd`
* Set `cgroupDriver: systemd` in `/var/lib/kubelet/config.yaml`
* Start the container runtime
* Start the kubelet using `systemctl start kubelet`