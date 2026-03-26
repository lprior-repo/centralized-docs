---
doc_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes
chunk_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes#9-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 91
summary: ### Upgrade kubelet and kube-proxy 1. From the Windows node, upgrade and restart the kubelet: ``` `stop-service kubelet curl.exe -Lo &lt;path-to-kubelet.exe&gt;...
---

### Upgrade kubelet and kube-proxy
1. From the Windows node, upgrade and restart the kubelet:
```
`stop-service kubelet
curl.exe -Lo &lt;path-to-kubelet.exe&gt; "https://dl.k8s.io/v1.35.0/bin/windows/amd64/kubelet.exe"
restart-service kubelet
`
```
2. From the Windows node, upgrade and restart the kube-proxy.