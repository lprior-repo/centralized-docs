---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#25-summary
chunk_level: summary
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 106
summary: volumeClaimTemplates: - metadata: name: cassandra-data spec: accessModes: [ \"ReadWriteOnce\" ] storageClassName: fast resources: requests: storage: 1Gi --- kind: StorageClass apiVersion:...
---

volumeClaimTemplates:
- metadata:
name: cassandra-data
spec:
accessModes: [ "ReadWriteOnce" ]
storageClassName: fast
resources:
requests:
storage: 1Gi
---
kind: StorageClass
apiVersion: storage.k8s.io/v1
metadata:
name: fast
provisioner: k8s.io/minikube-hostpath
parameters:
type: pd-ssd
`
```
Create the Cassandra StatefulSet from the `cassandra-statefulset.yaml` file: