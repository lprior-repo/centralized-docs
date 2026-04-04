---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#70-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 68
summary: #### Note: TLS bootstrapping for nodes can be configured with the command [`kubeadm init phase bootstrap-token`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-bootstrap-token),...
---

#### Note:
TLS bootstrapping for nodes can be configured with the command
[`kubeadm init phase bootstrap-token`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-bootstrap-token),
executing all the configuration steps described in following paragraphs;
alternatively, each step can be invoked individually.