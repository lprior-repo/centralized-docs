---
doc_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes
chunk_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes#3-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 322
summary: ### Upgrade the kubelet configuration 1. From the Windows node, call the following command to sync new kubelet configuration: ``` `kubeadm upgrade node ` ``` ### Upgrade kubelet and kube-proxy 1....
---

### Upgrade the kubelet configuration
1. From the Windows node, call the following command to sync new kubelet configuration:
```
`kubeadm upgrade node
`
```
### Upgrade kubelet and kube-proxy
1. From the Windows node, upgrade and restart the kubelet:
```
`stop-service kubelet
curl.exe -Lo &lt;path-to-kubelet.exe&gt; "https://dl.k8s.io/v1.35.0/bin/windows/amd64/kubelet.exe"
restart-service kubelet
`
```
2. From the Windows node, upgrade and restart the kube-proxy.
```
`stop-service kube-proxy
curl.exe -Lo &lt;path-to-kube-proxy.exe&gt; "https://dl.k8s.io/v1.35.0/bin/windows/amd64/kube-proxy.exe"
restart-service kube-proxy
`
```
#### Note:
If you are running kube-proxy in a HostProcess container within a Pod, and not as a Windows Service,
you can upgrade kube-proxy by applying a newer version of your kube-proxy manifests.
### Uncordon the node
1. From a machine with access to the Kubernetes API,
bring the node back online by marking it schedulable:
```
`# replace &lt;node-to-drain&gt; with the name of your node
kubectl uncordon &lt;node-to-drain&gt;
`
```
## What's next
* See how to [Upgrade Linux nodes](/docs/tasks/administer-cluster/kubeadm/upgrading-linux-nodes/).