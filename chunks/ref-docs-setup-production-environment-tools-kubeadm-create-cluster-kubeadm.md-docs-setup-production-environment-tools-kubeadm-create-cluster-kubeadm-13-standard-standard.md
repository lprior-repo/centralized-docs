---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#13-standard
chunk_level: standard
chunk_type: prose
heading: Objectives
token_count: 490
summary: ### Adding more control plane nodes See [Creating Highly Available Clusters with kubeadm](/docs/setup/production-environment/tools/kubeadm/high-availability/) for steps on creating a high...
---

### Adding more control plane nodes
See [Creating Highly Available Clusters with kubeadm](/docs/setup/production-environment/tools/kubeadm/high-availability/)
for steps on creating a high availability kubeadm cluster by adding more control plane nodes.
### Adding worker nodes
The worker nodes are where your workloads run.
The following pages show how to add Linux and Windows worker nodes to the cluster by using
the `kubeadm join` command:
* [Adding Linux worker nodes](/docs/tasks/administer-cluster/kubeadm/adding-linux-nodes/)
* [Adding Windows worker nodes](/docs/tasks/administer-cluster/kubeadm/adding-windows-nodes/)### (Optional) Controlling your cluster from machines other than the control-plane node
In order to get a kubectl on some other computer (e.g. laptop) to talk to your
cluster, you need to copy the administrator kubeconfig file from your control-plane node
to your workstation like this:
```
`scp root@&lt;control-plane-host&gt;:/etc/kubernetes/admin.conf .
kubectl --kubeconfig ./admin.conf get nodes
`
```
#### Note:
The example above assumes SSH access is enabled for root. If that is not the
case, you can copy the `admin.conf` file to be accessible by some other user
and `scp` using that other user instead.
The `admin.conf` file gives the user *superuser* privileges over the cluster.
This file should be used sparingly. For normal users, it's recommended to
generate an unique credential to which you grant privileges. You can do
this with the `kubeadm kubeconfig user --client-name &lt;CN&gt;`
command. That command will print out a KubeConfig file to STDOUT which you
should save to a file and distribute to your user. After that, grant
privileges by using `kubectl create (cluster)rolebinding`.
### (Optional) Proxying API Server to localhost
If you want to connect to the API Server from outside the cluster, you can use
`kubectl proxy`:
```
`scp root@&lt;control-plane-host&gt;:/etc/kubernetes/admin.conf .
kubectl --kubeconfig ./admin.conf proxy
`
```
You can now access the API Server locally at `http://localhost:8001/api/v1`