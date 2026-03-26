---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#30-summary
chunk_level: summary
chunk_type: prose
heading: Validating the Cassandra StatefulSet
token_count: 127
summary: It can take several minutes for all three Pods to deploy. Once they are deployed, the same command returns output similar to: ``` `NAME READY STATUS RESTARTS AGE cassandra-0 1/1 Running 0 10m...
---

It can take several minutes for all three Pods to deploy. Once they are deployed, the same command
returns output similar to:
```
`NAME READY STATUS RESTARTS AGE
cassandra-0 1/1 Running 0 10m
cassandra-1 1/1 Running 0 9m
cassandra-2 1/1 Running 0 8m
`
```
3. Run the Cassandra [nodetool](https://cwiki.apache.org/confluence/display/CASSANDRA2/NodeTool) inside the first Pod, to
display the status of the ring.