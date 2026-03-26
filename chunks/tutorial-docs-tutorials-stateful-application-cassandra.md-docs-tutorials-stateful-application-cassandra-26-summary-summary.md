---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#26-summary
chunk_level: summary
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 62
summary: Create the Cassandra StatefulSet from the `cassandra-statefulset.yaml` file: ``` `# Use this if you are able to apply cassandra-statefulset.yaml unmodified kubectl apply -f...
---

Create the Cassandra StatefulSet from the `cassandra-statefulset.yaml` file:
```
`# Use this if you are able to apply cassandra-statefulset.yaml unmodified
kubectl apply -f https://k8s.io/examples/application/cassandra/cassandra-statefulset.yaml
`
```