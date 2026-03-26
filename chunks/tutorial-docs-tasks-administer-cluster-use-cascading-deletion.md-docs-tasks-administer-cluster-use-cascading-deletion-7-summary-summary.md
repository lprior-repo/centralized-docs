---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#7-summary
chunk_level: summary
chunk_type: prose
heading: Check owner references on your pods
token_count: 96
summary: The output has an `ownerReferences` field similar to this: ``` `apiVersion: v1 ... ownerReferences: - apiVersion: apps/v1 blockOwnerDeletion: true controller: true kind: ReplicaSet name:...
---

The output has an `ownerReferences` field similar to this:
```
`apiVersion: v1
...
ownerReferences:
- apiVersion: apps/v1
blockOwnerDeletion: true
controller: true
kind: ReplicaSet
name: nginx-deployment-6b474476c4
uid: 4fdcd81c-bd5d-41f7-97af-3a3b759af9a7
...
`
```