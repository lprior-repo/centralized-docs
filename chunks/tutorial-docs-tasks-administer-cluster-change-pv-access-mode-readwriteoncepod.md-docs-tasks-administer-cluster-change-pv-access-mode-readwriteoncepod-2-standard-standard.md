---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#2-standard
chunk_level: standard
chunk_type: prose
heading: Why should I use `ReadWriteOncePod`?
token_count: 132
summary: ## Why should I use `ReadWriteOncePod`? Prior to Kubernetes v1.22, the `ReadWriteOnce` access mode was commonly used to restrict PersistentVolume access for workloads that required single-writer...
---

## Why should I use `ReadWriteOncePod`?
Prior to Kubernetes v1.22, the `ReadWriteOnce` access mode was commonly used to
restrict PersistentVolume access for workloads that required single-writer
access to storage. However, this access mode had a limitation: it restricted
volume access to a single *node*, allowing multiple pods on the same node to
read from and write to the same volume simultaneously. This could pose a risk
for applications that demand strict single-writer access for data safety.
If ensuring single-writer access is critical for your workloads, consider
migrating your volumes to `ReadWriteOncePod`.