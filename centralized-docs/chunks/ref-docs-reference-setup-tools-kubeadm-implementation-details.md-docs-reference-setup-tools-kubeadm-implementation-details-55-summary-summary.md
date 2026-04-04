---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#55-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 120
summary: * [`ServiceAccount`](/docs/reference/access-authn-authz/admission-controllers/#serviceaccount) to enforce service account automation *...
---

* [`ServiceAccount`](/docs/reference/access-authn-authz/admission-controllers/#serviceaccount)
to enforce service account automation
* [`PersistentVolumeLabel`](/docs/reference/access-authn-authz/admission-controllers/#persistentvolumelabel)
attaches region or zone labels to PersistentVolumes as defined by the cloud provider (This
admission controller is deprecated and will be removed in a future version.
It is not deployed by kubeadm by default with v1.9 onwards when not explicitly opting into
using `gce` or `aws` as cloud providers)