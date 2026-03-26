---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#21-summary
chunk_level: summary
chunk_type: prose
heading: Migrating to the `systemd` driver
token_count: 90
summary: * Start the container runtime * Start the kubelet using `systemctl start kubelet` * [Uncordon the node](/docs/tasks/administer-cluster/safely-drain-node/) using `kubectl uncordon &lt;node-name&gt;`...
---

* Start the container runtime
* Start the kubelet using `systemctl start kubelet`
* [Uncordon the node](/docs/tasks/administer-cluster/safely-drain-node/) using `kubectl uncordon &lt;node-name&gt;`
Execute these steps on nodes one at a time to ensure workloads
have sufficient time to schedule on different nodes.
Once the process is complete ensure that all nodes and workloads are healthy.