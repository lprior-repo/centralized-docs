---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#54-summary
chunk_level: summary
chunk_type: prose
heading: Manual certificate distribution
token_count: 121
summary: ## Manual certificate distribution If you choose to not use `kubeadm init` with the `--upload-certs` flag this means that you are going to have to manually copy the certificates from the primary...
---

## Manual certificate distribution
If you choose to not use `kubeadm init` with the `--upload-certs` flag this means that
you are going to have to manually copy the certificates from the primary control plane node to the
joining control plane nodes.
There are many ways to do this. The following example uses `ssh` and `scp`:
SSH is required if you want to control all nodes from a single machine.
1. Enable ssh-agent on your main device that has access to all other nodes in
the system:
```
`eval $(ssh-agent)
`
```