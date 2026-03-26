---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#16-summary
chunk_level: summary
chunk_type: prose
heading: Creating a headless Service for Cassandra
token_count: 106
summary: ### Validating (optional) Get the Cassandra Service. ``` `kubectl get svc cassandra ` ``` The response is ``` `NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE cassandra ClusterIP None &lt;none&gt;...
---

### Validating (optional)
Get the Cassandra Service.
```
`kubectl get svc cassandra
`
```
The response is
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
cassandra ClusterIP None &lt;none&gt; 9042/TCP 45s
`
```
If you don't see a Service named `cassandra`, that means creation failed. Read
[Debug Services](/docs/tasks/debug/debug-application/debug-service/)
for help troubleshooting common issues.