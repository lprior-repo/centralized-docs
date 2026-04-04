---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#25-summary
chunk_level: summary
chunk_type: prose
heading: Audit backends
token_count: 116
summary: #### Note: In case of patches, request body is a JSON array with patch operations, not a JSON object with an appropriate Kubernetes API object. For example, the following request body is a valid...
---

#### Note:
In case of patches, request body is a JSON array with patch operations, not a JSON object
with an appropriate Kubernetes API object. For example, the following request body is a valid patch
request to `/apis/batch/v1/namespaces/some-namespace/jobs/some-job-name`:
```
`[
{
"op": "replace",
"path": "/spec/parallelism",
"value": 0
},
{
"op": "remove",
"path": "/spec/template/spec/containers/0/terminationMessagePolicy"
}
]
`
```