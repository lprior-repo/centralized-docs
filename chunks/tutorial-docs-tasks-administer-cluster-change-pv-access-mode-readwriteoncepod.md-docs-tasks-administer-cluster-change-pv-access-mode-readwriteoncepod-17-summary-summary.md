---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#17-summary
chunk_level: summary
chunk_type: prose
heading: Migrating existing PersistentVolumes
token_count: 121
summary: - name: cat-pictures mountPath: /mnt volumes: - name: cat-pictures persistentVolumeClaim: claimName: cat-pictures-pvc readOnly: false ` ``` As a first step, you need to edit your PersistentVolume's...
---

- name: cat-pictures
mountPath: /mnt
volumes:
- name: cat-pictures
persistentVolumeClaim:
claimName: cat-pictures-pvc
readOnly: false
`
```
As a first step, you need to edit your PersistentVolume's
`spec.persistentVolumeReclaimPolicy` and set it to `Retain`. This ensures your
PersistentVolume will not be deleted when you delete the corresponding
PersistentVolumeClaim:
```
`kubectl patch pv cat-pictures-pv -p '{"spec":{"persistentVolumeReclaimPolicy":"Retain"}}'
`
```