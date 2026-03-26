---
doc_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes
chunk_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes#10-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 110
summary: 2. From the Windows node, upgrade and restart the kube-proxy. ``` `stop-service kube-proxy curl.exe -Lo &lt;path-to-kube-proxy.exe&gt; \"https://dl.k8s.io/v1.35.0/bin/windows/amd64/kube-proxy.exe\"...
---

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