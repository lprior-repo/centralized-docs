---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#24-summary
chunk_level: summary
chunk_type: prose
heading: Using a StatefulSet to create a Cassandra ring
token_count: 126
summary: initialDelaySeconds: 15 timeoutSeconds: 5 # These volume mounts are persistent. They are like inline claims, # but not exactly because the names need to match exactly one of # the stateful pod...
---

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