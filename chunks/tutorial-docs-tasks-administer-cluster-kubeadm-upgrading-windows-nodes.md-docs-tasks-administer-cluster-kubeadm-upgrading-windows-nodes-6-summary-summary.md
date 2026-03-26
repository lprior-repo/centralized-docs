---
doc_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes
chunk_id: tutorial/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes.md/docs-tasks-administer-cluster-kubeadm-upgrading-windows-nodes#6-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 79
summary: ### Upgrade kubeadm 1. From the Windows node, upgrade kubeadm: ``` `# replace 1.35.0 with your desired version curl.exe -Lo &lt;path-to-kubeadm.exe&gt;...
---

### Upgrade kubeadm
1. From the Windows node, upgrade kubeadm:
```
`# replace 1.35.0 with your desired version
curl.exe -Lo &lt;path-to-kubeadm.exe&gt; "https://dl.k8s.io/v1.35.0/bin/windows/amd64/kubeadm.exe"
`
```