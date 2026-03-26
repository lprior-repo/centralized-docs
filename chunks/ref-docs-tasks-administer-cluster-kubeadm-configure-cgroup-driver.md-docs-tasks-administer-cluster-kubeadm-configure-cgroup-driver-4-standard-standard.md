---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#4-standard
chunk_level: standard
chunk_type: prose
heading: Migrating to the `systemd` driver
token_count: 389
summary: ## Migrating to the `systemd` driver To change the cgroup driver of an existing kubeadm cluster from `cgroupfs` to `systemd` in-place, a similar procedure to a kubelet upgrade is required. This must...
---

## Migrating to the `systemd` driver
To change the cgroup driver of an existing kubeadm cluster from `cgroupfs` to `systemd` in-place,
a similar procedure to a kubelet upgrade is required. This must include both
steps outlined below.
#### Note:
Alternatively, it is possible to replace the old nodes in the cluster with new ones
that use the `systemd` driver. This requires executing only the first step below
before joining the new nodes and ensuring the workloads can safely move to the new
nodes before deleting the old nodes.
### Modify the kubelet ConfigMap
* Call `kubectl edit cm kubelet-config -n kube-system`.
* Either modify the existing `cgroupDriver` value or add a new field that looks like this:
```
`cgroupDriver: systemd
`
```
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