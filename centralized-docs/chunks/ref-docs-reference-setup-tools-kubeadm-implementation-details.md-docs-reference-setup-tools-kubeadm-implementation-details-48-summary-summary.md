---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#48-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 84
summary: * Controller-manager and the scheduler will reference kubeconfig files with their respective, unique identities * All static Pods get any extra flags or patches that you specify, as described in...
---

* Controller-manager and the scheduler will reference kubeconfig files with their respective, unique identities
* All static Pods get any extra flags or patches that you specify, as described in
[passing custom arguments to control plane components](/docs/setup/production-environment/tools/kubeadm/control-plane-flags/)
* All static Pods get any extra Volumes specified by the user (Host path)
Please note that: