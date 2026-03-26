---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#11-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 115
summary: Endpoints: 10.200.1.4:8080,10.200.2.5:8080 Session Affinity: None Events: &lt;none&gt; ` ``` Make a note of the NodePort value for the Service. For example, in the preceding output, the NodePort...
---

Endpoints: 10.200.1.4:8080,10.200.2.5:8080
Session Affinity: None
Events: &lt;none&gt;
`
```
Make a note of the NodePort value for the Service. For example,
in the preceding output, the NodePort value is 31496.
6. List the pods that are running the Hello World application:
```
`kubectl get pods --selector="run=load-balancer-example" --output=wide
`
```
The output is similar to this: