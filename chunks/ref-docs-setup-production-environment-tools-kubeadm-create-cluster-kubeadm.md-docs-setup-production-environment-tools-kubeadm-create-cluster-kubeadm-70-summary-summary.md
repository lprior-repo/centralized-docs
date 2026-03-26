---
doc_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm
chunk_id: ref/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md/docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm#70-summary
chunk_level: summary
chunk_type: prose
heading: Objectives
token_count: 86
summary: ### (Optional) Proxying API Server to localhost If you want to connect to the API Server from outside the cluster, you can use `kubectl proxy`: ``` `scp...
---

### (Optional) Proxying API Server to localhost
If you want to connect to the API Server from outside the cluster, you can use
`kubectl proxy`:
```
`scp root@&lt;control-plane-host&gt;:/etc/kubernetes/admin.conf .
kubectl --kubeconfig ./admin.conf proxy
`
```
You can now access the API Server locally at `http://localhost:8001/api/v1`