---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#27-summary
chunk_level: summary
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 104
summary: If you need to modify `cassandra-statefulset.yaml` to suit your cluster, download...
---

If you need to modify `cassandra-statefulset.yaml` to suit your cluster, download
[https://k8s.io/examples/application/cassandra/cassandra-statefulset.yaml](https://k8s.io/examples/application/cassandra/cassandra-statefulset.yaml) and then apply
that manifest, from the folder you saved the modified version into:
```
`# Use this if you needed to modify cassandra-statefulset.yaml locally
kubectl apply -f cassandra-statefulset.yaml
`
```