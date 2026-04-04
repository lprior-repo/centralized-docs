---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#7-standard
chunk_level: standard
chunk_type: prose
heading: Audit backends
token_count: 221
summary: ## Audit backends Audit backends persist audit events to an external storage. Out of the box, the kube-apiserver provides two backends: * Log backend, which writes events into the filesystem *...
---

## Audit backends
Audit backends persist audit events to an external storage.
Out of the box, the kube-apiserver provides two backends:
* Log backend, which writes events into the filesystem
* Webhook backend, which sends events to an external HTTP API
In all cases, audit events follow a structure defined by the Kubernetes API in the
[`audit.k8s.io` API group](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Event).
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