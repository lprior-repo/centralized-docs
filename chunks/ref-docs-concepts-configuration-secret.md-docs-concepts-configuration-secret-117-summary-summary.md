---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#117-summary
chunk_level: summary
chunk_type: prose
heading: Immutable Secrets
token_count: 110
summary: * protects you from accidental (or unwanted) updates that could cause applications outages * (for clusters that extensively use Secrets - at least tens of thousands of unique Secret to Pod mounts),...
---

* protects you from accidental (or unwanted) updates that could cause applications outages
* (for clusters that extensively use Secrets - at least tens of thousands of unique Secret
to Pod mounts), switching to immutable Secrets improves the performance of your cluster
by significantly reducing load on kube-apiserver. The kubelet does not need to maintain
a [watch] on any Secrets that are marked as immutable.### Marking a Secret as immutable
You can create an immutable Secret by setting the `immutable` field to `true`. For example,