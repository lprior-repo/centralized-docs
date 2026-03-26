---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#20-summary
chunk_level: summary
chunk_type: prose
heading: Migrating existing PersistentVolumes
token_count: 121
summary: #### Note: The `ReadWriteOncePod` access mode cannot be combined with other access modes. Make sure `ReadWriteOncePod` is the only access mode on the PersistentVolume when updating, otherwise the...
---

#### Note:
The `ReadWriteOncePod` access mode cannot be combined with other access modes.
Make sure `ReadWriteOncePod` is the only access mode on the PersistentVolume
when updating, otherwise the request will fail.
Next you need to modify your PersistentVolumeClaim to set `ReadWriteOncePod` as
the only access mode. You should also set the PersistentVolumeClaim's
`spec.volumeName` to the name of your PersistentVolume to ensure it binds to
this specific PersistentVolume.
Once this is done, you can recreate your PersistentVolumeClaim and start up your
workloads: