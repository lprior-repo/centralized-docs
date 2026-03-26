---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#20-summary
chunk_level: summary
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 128
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