---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#15-summary
chunk_level: summary
chunk_type: prose
heading: Creating a headless Service for Cassandra
token_count: 98
summary: ``` `apiVersion: v1 kind: Service metadata: labels: app: cassandra name: cassandra spec: clusterIP: None ports: - port: 9042 selector: app: cassandra ` ``` Create a Service to track all Cassandra...
---

```
`apiVersion: v1
kind: Service
metadata:
labels:
app: cassandra
name: cassandra
spec:
clusterIP: None
ports:
- port: 9042
selector:
app: cassandra
`
```
Create a Service to track all Cassandra StatefulSet members from the `cassandra-service.yaml` file:
```
`kubectl apply -f https://k8s.io/examples/application/cassandra/cassandra-service.yaml
`
```