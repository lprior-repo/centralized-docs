---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#27-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 121
summary: * [Error] if ports 2379 is used * [Error] if Etcd.DataDir folder already exists and it is not empty * If authorization mode is ABAC: * [Error] if abac\_policy.json does not exist * If authorization...
---

* [Error] if ports 2379 is used
* [Error] if Etcd.DataDir folder already exists and it is not empty
* If authorization mode is ABAC:
* [Error] if abac\_policy.json does not exist
* If authorization mode is WebHook
* [Error] if webhook\_authz.conf does not exist
#### Note:
Preflight checks can be invoked individually with the
[`kubeadm init phase preflight`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-preflight)
command.