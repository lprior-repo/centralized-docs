---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#3-standard
chunk_level: standard
chunk_type: prose
heading: Migrating existing PersistentVolumes
token_count: 97
summary: ## Migrating existing PersistentVolumes If you have existing PersistentVolumes, they can be migrated to use `ReadWriteOncePod`. Only migrations from `ReadWriteOnce` to `ReadWriteOncePod` are...
---

## Migrating existing PersistentVolumes
If you have existing PersistentVolumes, they can be migrated to use
`ReadWriteOncePod`. Only migrations from `ReadWriteOnce` to `ReadWriteOncePod`
are supported.
In this example, there is already a `ReadWriteOnce` "cat-pictures-pvc"
PersistentVolumeClaim that is bound to a "cat-pictures-pv" PersistentVolume,
and a "cat-pictures-writer" Deployment that uses this PersistentVolumeClaim.