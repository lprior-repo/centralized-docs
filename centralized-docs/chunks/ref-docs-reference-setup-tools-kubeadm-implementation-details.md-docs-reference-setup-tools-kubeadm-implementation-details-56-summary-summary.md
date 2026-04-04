---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#56-summary
chunk_level: summary
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 104
summary: * [`DefaultStorageClass`](/docs/reference/access-authn-authz/admission-controllers/#defaultstorageclass) to enforce default storage class on `PersistentVolumeClaim` objects *...
---

* [`DefaultStorageClass`](/docs/reference/access-authn-authz/admission-controllers/#defaultstorageclass)
to enforce default storage class on `PersistentVolumeClaim` objects
* [`DefaultTolerationSeconds`](/docs/reference/access-authn-authz/admission-controllers/#defaulttolerationseconds)
* [`NodeRestriction`](/docs/reference/access-authn-authz/admission-controllers/#noderestriction)
to limit what a kubelet can modify (e.g. only pods on this node)