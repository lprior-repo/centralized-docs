---
doc_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion
chunk_id: tutorial/docs-tasks-administer-cluster-use-cascading-deletion.md/docs-tasks-administer-cluster-use-cascading-deletion#16-summary
chunk_level: summary
chunk_type: prose
heading: Use background cascading deletion
token_count: 85
summary: The output is similar to this: ``` `\"kind\": \"Status\", \"apiVersion\": \"v1\", ... \"status\": \"Success\", \"details\": { \"name\": \"nginx-deployment\", \"group\": \"apps\", \"kind\": \"deployments\", \"uid\":...
---

The output is similar to this:
```
`"kind": "Status",
"apiVersion": "v1",
...
"status": "Success",
"details": {
"name": "nginx-deployment",
"group": "apps",
"kind": "deployments",
"uid": "cc9eefb9-2d49-4445-b1c1-d261c9396456"
}
`
```