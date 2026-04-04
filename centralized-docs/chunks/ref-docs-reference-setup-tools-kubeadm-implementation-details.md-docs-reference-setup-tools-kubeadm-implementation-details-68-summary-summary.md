---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#68-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 92
summary: * Labels the node as control-plane with `node-role.kubernetes.io/control-plane=\"\"` * Taints the node with `node-role.kubernetes.io/control-plane:NoSchedule` Please note that the phase to mark the...
---

* Labels the node as control-plane with `node-role.kubernetes.io/control-plane=""`
* Taints the node with `node-role.kubernetes.io/control-plane:NoSchedule`
Please note that the phase to mark the control-plane phase can be invoked
individually with the [`kubeadm init phase mark-control-plane`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-mark-control-plane) command.