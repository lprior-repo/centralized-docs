---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#13-summary
chunk_level: summary
chunk_type: prose
heading: Migrating existing PersistentVolumes
token_count: 127
summary: #### Note: If your storage plugin supports [Dynamic provisioning](/docs/concepts/storage/dynamic-provisioning/), the \"cat-picutres-pv\" will be created for you, but its name may differ. To get your...
---

#### Note:
If your storage plugin supports
[Dynamic provisioning](/docs/concepts/storage/dynamic-provisioning/),
the "cat-picutres-pv" will be created for you, but its name may differ. To get
your PersistentVolume's name run:
```
`kubectl get pvc cat-pictures-pvc -o jsonpath='{.spec.volumeName}'
`
```
And you can view the PVC before you make changes. Either view the manifest
locally, or run `kubectl get pvc &lt;name-of-pvc&gt; -o yaml`. The output is similar
to: