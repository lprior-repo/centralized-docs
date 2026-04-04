---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#64-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 115
summary: 3. You can directly invoke static Pod manifest generation for local etcd, using the [`kubeadm init phase etcd local`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-etcd)...
---

3. You can directly invoke static Pod manifest generation for local etcd, using the
[`kubeadm init phase etcd local`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-etcd)
command.### Wait for the control plane to come up
On control plane nodes, kubeadm waits up to 4 minutes for the control plane components
and the kubelet to be available. It does that by performing a health check on the respective
component `/healthz` or `/livez` endpoints.