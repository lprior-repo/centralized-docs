---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#8-standard
chunk_level: standard
chunk_type: code
heading: Upgrading control plane nodes
token_count: 449
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