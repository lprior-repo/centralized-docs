---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#14-summary
chunk_level: summary
chunk_type: prose
heading: Migrating existing PersistentVolumes
token_count: 69
summary: ``` `# cat-pictures-pvc.yaml kind: PersistentVolumeClaim apiVersion: v1 metadata: name: cat-pictures-pvc spec: accessModes: - ReadWriteOnce resources: requests: storage: 1Gi ` ``` Here's an example...
---

```
`# cat-pictures-pvc.yaml
kind: PersistentVolumeClaim
apiVersion: v1
metadata:
name: cat-pictures-pvc
spec:
accessModes:
- ReadWriteOnce
resources:
requests:
storage: 1Gi
`
```
Here's an example Deployment that relies on that PersistentVolumeClaim: