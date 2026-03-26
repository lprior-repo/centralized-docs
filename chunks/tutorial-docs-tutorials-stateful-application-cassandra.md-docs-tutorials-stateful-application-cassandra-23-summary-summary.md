---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#23-summary
chunk_level: summary
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 118
summary: value: \"K8Demo\" - name: CASSANDRA\_DC value: \"DC1-K8Demo\" - name: CASSANDRA\_RACK value: \"Rack1-K8Demo\" - name: POD\_IP valueFrom: fieldRef: fieldPath: status.podIP readinessProbe: exec: command: -...
---

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