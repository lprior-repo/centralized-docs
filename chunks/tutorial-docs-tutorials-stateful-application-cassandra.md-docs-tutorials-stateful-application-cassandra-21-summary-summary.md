---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#21-summary
chunk_level: summary
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 125
summary: ports: - containerPort: 7000 name: intra-node - containerPort: 7001 name: tls-intra-node - containerPort: 7199 name: jmx - containerPort: 9042 name: cql resources: limits: cpu: \"500m\" memory: 1Gi...
---

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