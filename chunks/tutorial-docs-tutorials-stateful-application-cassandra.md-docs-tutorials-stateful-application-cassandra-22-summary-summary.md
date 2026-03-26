---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#22-summary
chunk_level: summary
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 123
summary: lifecycle: preStop: exec: command: - /bin/sh - -c - nodetool drain env: - name: MAX\_HEAP\_SIZE value: 512M - name: HEAP\_NEWSIZE value: 100M - name: CASSANDRA\_SEEDS value:...
---

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