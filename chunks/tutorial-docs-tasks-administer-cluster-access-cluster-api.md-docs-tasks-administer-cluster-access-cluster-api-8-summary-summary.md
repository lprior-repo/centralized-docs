---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#8-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 117
summary: ### Accessing for the first time with kubectl When accessing the Kubernetes API for the first time, use the Kubernetes command-line tool, `kubectl`. To access a cluster, you need to know the location...
---

### Accessing for the first time with kubectl
When accessing the Kubernetes API for the first time, use the
Kubernetes command-line tool, `kubectl`.
To access a cluster, you need to know the location of the cluster and have credentials
to access it. Typically, this is automatically set-up when you work through
a [Getting started guide](/docs/setup/),
or someone else set up the cluster and provided you with credentials and a location.
Check the location and credentials that kubectl knows about with this command:
```
`kubectl config view
`
```