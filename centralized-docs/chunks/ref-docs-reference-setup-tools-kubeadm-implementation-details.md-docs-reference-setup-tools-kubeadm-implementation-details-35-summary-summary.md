---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#35-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 96
summary: 4. In case kubeadm is executed in the `--dry-run` mode, certificate files are written in a temporary folder 5. Certificate generation can be invoked individually with the [`kubeadm init phase certs...
---

4. In case kubeadm is executed in the `--dry-run` mode, certificate files are written in a temporary folder
5. Certificate generation can be invoked individually with the
[`kubeadm init phase certs all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-certs) command### Generate kubeconfig files for control plane components
Kubeadm generates kubeconfig files with identities for control plane components: