---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#7-standard
chunk_level: standard
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 512
summary: `apiVersion: apps/v1 kind: StatefulSet metadata: name: cassandra labels: app: cassandra spec: serviceName: cassandra replicas: 3 selector: matchLabels: app: cassandra template: metadata: labels: app:...
---

`apiVersion: apps/v1
kind: StatefulSet
metadata:
name: cassandra
labels:
app: cassandra
spec:
serviceName: cassandra
replicas: 3
selector:
matchLabels:
app: cassandra
template:
metadata:
labels:
app: cassandra
spec:
terminationGracePeriodSeconds: 500
containers:
- name: cassandra
image: gcr.io/google-samples/cassandra:v13
imagePullPolicy: Always
ports:
- containerPort: 7000
name: intra-node
- containerPort: 7001
name: tls-intra-node
- containerPort: 7199
name: jmx
- containerPort: 9042
name: cql
resources:
limits:
cpu: "500m"
memory: 1Gi
requests:
cpu: "500m"
memory: 1Gi
securityContext:
capabilities:
add:
- IPC\_LOCK
lifecycle:
preStop:
exec:
command:
- /bin/sh
- -c
- nodetool drain
env:
- name: MAX\_HEAP\_SIZE
value: 512M
- name: HEAP\_NEWSIZE
value: 100M
- name: CASSANDRA\_SEEDS
value: "cassandra-0.cassandra.default.svc.cluster.local"
- name: CASSANDRA\_CLUSTER\_NAME
value: "K8Demo"
- name: CASSANDRA\_DC
value: "DC1-K8Demo"
- name: CASSANDRA\_RACK
value: "Rack1-K8Demo"
- name: POD\_IP
valueFrom:
fieldRef:
fieldPath: status.podIP
readinessProbe:
exec:
command:
- /bin/bash
- -c
- /ready-probe.sh
initialDelaySeconds: 15
timeoutSeconds: 5
# These volume mounts are persistent. They are like inline claims,
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