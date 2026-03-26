---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#19-summary
chunk_level: summary
chunk_type: prose
heading: Delete owner objects and orphan dependents
token_count: 127
summary: ``` `\"kind\": \"Deployment\", \"apiVersion\": \"apps/v1\", \"namespace\": \"default\", \"uid\": \"6f577034-42a0-479d-be21-78018c466f1f\", \"creationTimestamp\": \"2021-07-09T16:46:37Z\", \"deletionTimestamp\":...
---

```
`"kind": "Deployment",
"apiVersion": "apps/v1",
"namespace": "default",
"uid": "6f577034-42a0-479d-be21-78018c466f1f",
"creationTimestamp": "2021-07-09T16:46:37Z",
"deletionTimestamp": "2021-07-09T16:47:08Z",
"deletionGracePeriodSeconds": 0,
"finalizers": [
"orphan"
],
...
`
```
You can check that the Pods managed by the Deployment are still running: