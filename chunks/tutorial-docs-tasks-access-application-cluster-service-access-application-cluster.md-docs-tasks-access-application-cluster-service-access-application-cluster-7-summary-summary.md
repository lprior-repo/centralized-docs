---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#7-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 121
summary: 1. Run a Hello World application in your cluster: Create the application Deployment using the file above: ``` `kubectl apply -f https://k8s.io/examples/service/access/hello-application.yaml ` ``` The...
---

1. Run a Hello World application in your cluster:
Create the application Deployment using the file above:
```
`kubectl apply -f https://k8s.io/examples/service/access/hello-application.yaml
`
```
The preceding command creates a
[Deployment](/docs/concepts/workloads/controllers/deployment/)
and an associated
[ReplicaSet](/docs/concepts/workloads/controllers/replicaset/).
The ReplicaSet has two
[Pods](/docs/concepts/workloads/pods/)
each of which runs the Hello World application.
2. Display information about the Deployment: