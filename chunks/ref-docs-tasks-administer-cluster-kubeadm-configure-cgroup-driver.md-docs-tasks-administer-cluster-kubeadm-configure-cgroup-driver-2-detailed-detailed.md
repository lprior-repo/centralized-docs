---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 867
summary: ## Using the `cgroupfs` driver To use `cgroupfs` and to prevent `kubeadm upgrade` from modifying the `KubeletConfiguration` cgroup driver on existing setups, you must be explicit about its value....
---

## Using the `cgroupfs` driver
To use `cgroupfs` and to prevent `kubeadm upgrade` from modifying the
`KubeletConfiguration` cgroup driver on existing setups, you must be explicit
about its value. This applies to a case where you do not wish future versions
of kubeadm to apply the `systemd` driver by default.
See the below section on "[Modify the kubelet ConfigMap](#modify-the-kubelet-configmap)" for details on
how to be explicit about the value.
If you wish to configure a container runtime to use the `cgroupfs` driver,
you must refer to the documentation of the container runtime of your choice.
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

- [Secrets](docs-concepts-configuration-secret.md)
- [Debugging DNS Resolution](docs-tasks-administer-cluster-dns-debugging-resolution.md)
- [Configuring each kubelet in your cluster using kubeadm](docs-setup-production-environment-tools-kubeadm-kubelet-integration.md)
- [Reserve Compute Resources for System Daemons](docs-tasks-administer-cluster-reserve-compute-resources.md)
- [Switching from Polling to CRI Event-based Updates to Container Status](docs-tasks-administer-cluster-switch-to-evented-pleg.md)