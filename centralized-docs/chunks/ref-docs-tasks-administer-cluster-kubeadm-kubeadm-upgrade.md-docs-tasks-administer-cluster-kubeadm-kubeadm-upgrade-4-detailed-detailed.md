---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#4-detailed
chunk_level: detailed
chunk_type: code
heading: Recovering from a failure state
token_count: 985
summary: ### Drain the node Prepare the node for maintenance by marking it unschedulable and evicting the workloads: ``` `# replace &lt;node-to-drain&gt; with the name of your node you are draining kubectl...
---

### Drain the node
Prepare the node for maintenance by marking it unschedulable and evicting the workloads:
```
`# replace &lt;node-to-drain&gt; with the name of your node you are draining
kubectl drain &lt;node-to-drain&gt; --ignore-daemonsets
`
```
#### Note:
On Linux nodes, the kubelet defaults to supporting only cgroups v2.
For Kubernetes 1.35 the `FailCgroupV1` kubelet configuration option is set to `true` by default.
To learn more, refer to the [Kubernetes cgroup v1 deprecation documentation](/docs/concepts/architecture/cgroups/#deprecation-of-cgroup-v1).
1. Upgrade the kubelet and kubectl:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo apt-mark unhold kubelet kubectl &amp;&amp; \\
sudo apt-get update &amp;&amp; sudo apt-get install -y kubelet='1.35.x-\*' kubectl='1.35.x-\*' &amp;&amp; \\
sudo apt-mark hold kubelet kubectl
`
```
For systems with DNF:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo yum install -y kubelet-'1.35.x-\*' kubectl-'1.35.x-\*' --disableexcludes=kubernetes
`
```
For systems with DNF5:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo yum install -y kubelet-'1.35.x-\*' kubectl-'1.35.x-\*' --setopt=disable\_excludes=kubernetes
`
```
2. Restart the kubelet:
```
`sudo systemctl daemon-reload
sudo systemctl restart kubelet
`
```
### Uncordon the node
Bring the node back online by marking it schedulable:
```
`# replace &lt;node-to-uncordon&gt; with the name of your node
kubectl uncordon &lt;node-to-uncordon&gt;
`
```
## Upgrade worker nodes
The upgrade procedure on worker nodes should be executed one node at a time or few nodes at a time,
without compromising the minimum required capacity for running your workloads.
The following pages show how to upgrade Linux and Windows worker nodes:
* [Upgrade Linux nodes](/docs/tasks/administer-cluster/kubeadm/upgrading-linux-nodes/)
* [Upgrade Windows nodes](/docs/tasks/administer-cluster/kubeadm/upgrading-windows-nodes/)## Verify the status of the cluster
After the kubelet is upgraded on all nodes verify that all nodes are available again by running
the following command from anywhere kubectl can access the cluster:
```
`kubectl get nodes
`
```
The `STATUS` column should show `Ready` for all your nodes, and the version number should be updated.
## Recovering from a failure state
If `kubeadm upgrade` fails and does not roll back, for example because of an unexpected shutdown during execution, you can run `kubeadm upgrade` again.
This command is idempotent and eventually makes sure that the actual state is the desired state you declare.
To recover from a bad state, you can also run `sudo kubeadm upgrade apply --force` without changing the version that your cluster is running.
During upgrade kubeadm writes the following backup folders under `/etc/kubernetes/tmp`:
* `kubeadm-backup-etcd-&lt;date&gt;-&lt;time&gt;`
* `kubeadm-backup-manifests-&lt;date&gt;-&lt;time&gt;`
`kubeadm-backup-etcd` contains a backup of the local etcd member data for this control plane Node.
In case of an etcd upgrade failure and if the automatic rollback does not work, the contents of this folder
can be manually restored in `/var/lib/etcd`. In case external etcd is used this backup folder will be empty.
`kubeadm-backup-manifests` contains a backup of the static Pod manifest files for this control plane Node.
In case of a upgrade failure and if the automatic rollback does not work, the contents of this folder can be
manually restored in `/etc/kubernetes/manifests`. If for some reason there is no difference between a pre-upgrade
and post-upgrade manifest file for a certain component, a backup file for it will not be written.
#### Note:
After the cluster upgrade using kubeadm, the backup directory `/etc/kubernetes/tmp` will remain and
these backup files will need to be cleared manually.