---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#8-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 72
summary: #### Note: The `ReadWriteOncePod` access mode is only supported for [CSI](/docs/concepts/storage/volumes/#csi) volumes. To use this volume access mode you will need to update the following [CSI...
---

#### Note:
The `ReadWriteOncePod` access mode is only supported for
[CSI](/docs/concepts/storage/volumes/#csi) volumes.
To use this volume access mode you will need to update the following
[CSI sidecars](https://kubernetes-csi.github.io/docs/sidecar-containers.html)
to these versions or greater: