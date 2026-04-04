---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#54-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 77
summary: * [`LimitRanger`](/docs/reference/access-authn-authz/admission-controllers/#limitranger) and [`ResourceQuota`](/docs/reference/access-authn-authz/admission-controllers/#resourcequota) to enforce...
---

* [`LimitRanger`](/docs/reference/access-authn-authz/admission-controllers/#limitranger)
and [`ResourceQuota`](/docs/reference/access-authn-authz/admission-controllers/#resourcequota)
to enforce limits on namespaces
* [`ServiceAccount`](/docs/reference/access-authn-authz/admission-controllers/#serviceaccount)
to enforce service account automation