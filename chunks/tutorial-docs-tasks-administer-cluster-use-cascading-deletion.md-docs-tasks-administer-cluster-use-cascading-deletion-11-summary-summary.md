---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#11-summary
chunk_level: summary
chunk_type: prose
heading: Use foreground cascading deletion
token_count: 126
summary: ``` `\"kind\": \"Deployment\", \"apiVersion\": \"apps/v1\", \"metadata\": { \"name\": \"nginx-deployment\", \"namespace\": \"default\", \"uid\": \"d1ce1b02-cae8-4288-8a53-30e84d8fa505\", \"resourceVersion\": \"1363097\",...
---

```
`"kind": "Deployment",
"apiVersion": "apps/v1",
"metadata": {
"name": "nginx-deployment",
"namespace": "default",
"uid": "d1ce1b02-cae8-4288-8a53-30e84d8fa505",
"resourceVersion": "1363097",
"creationTimestamp": "2021-07-08T20:24:37Z",
"deletionTimestamp": "2021-07-08T20:27:39Z",
"finalizers": [
"foregroundDeletion"
]
...
`
```