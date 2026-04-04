---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#5-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 136
summary: ## kubeadm init workflow internal design The `kubeadm init` consists of a sequence of atomic work tasks to perform, as described in the `kubeadm init` [internal...
---

## kubeadm init workflow internal design
The `kubeadm init` consists of a sequence of atomic work tasks to perform,
as described in the `kubeadm init` [internal workflow](/docs/reference/setup-tools/kubeadm/kubeadm-init/#init-workflow).
The [`kubeadm init phase`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/) command allows
users to invoke each task individually, and ultimately offers a reusable and composable
API/toolbox that can be used by other Kubernetes bootstrap tools, by any IT automation tool or by
an advanced user for creating custom clusters.