---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#19-summary
chunk_level: summary
chunk_type: prose
heading: Migrating existing PersistentVolumes
token_count: 102
summary: ``` `kubectl scale --replicas=0 deployment cat-pictures-writer kubectl delete pvc cat-pictures-pvc kubectl patch pv cat-pictures-pv -p '{\"spec\":{\"claimRef\":{\"uid\":\"\"}}}' ` ``` After that, replace the...
---

```
`kubectl scale --replicas=0 deployment cat-pictures-writer
kubectl delete pvc cat-pictures-pvc
kubectl patch pv cat-pictures-pv -p '{"spec":{"claimRef":{"uid":""}}}'
`
```
After that, replace the PersistentVolume's list of valid access modes to be
(only) `ReadWriteOncePod`:
```
`kubectl patch pv cat-pictures-pv -p '{"spec":{"accessModes":["ReadWriteOncePod"]}}'
`
```