---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#8-standard
chunk_level: standard
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 327
summary: # but not exactly because the names need to match exactly one of # the stateful pod volumes. volumeMounts: - name: cassandra-data mountPath: /cassandra\_data # These are converted to volume claims by...
---

# but not exactly because the names need to match exactly one of
# the stateful pod volumes.
volumeMounts:
- name: cassandra-data
mountPath: /cassandra\_data
# These are converted to volume claims by the controller
# do not use these in production until ssd GCEPersistentDisk or other ssd pd
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
```
`# Use this if you are able to apply cassandra-statefulset.yaml unmodified
kubectl apply -f https://k8s.io/examples/application/cassandra/cassandra-statefulset.yaml
`
```
If you need to modify `cassandra-statefulset.yaml` to suit your cluster, download
[https://k8s.io/examples/application/cassandra/cassandra-statefulset.yaml](https://k8s.io/examples/application/cassandra/cassandra-statefulset.yaml) and then apply
that manifest, from the folder you saved the modified version into:
```
`# Use this if you needed to modify cassandra-statefulset.yaml locally
kubectl apply -f cassandra-statefulset.yaml
`
```