---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#87-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm join phases internal design
token_count: 101
summary: 2. If you are joining a Windows node, Linux specific controls are skipped. 3. In any case the user can skip specific preflight checks (or eventually all preflight checks) with the...
---

2. If you are joining a Windows node, Linux specific controls are skipped.
3. In any case the user can skip specific preflight checks (or eventually all preflight checks)
with the `--ignore-preflight-errors` option.### Discovery cluster-info
There are 2 main schemes for discovery. The first is to use a shared token along with the IP
address of the API server.
The second is to provide a file (that is a subset of the standard kubeconfig file).