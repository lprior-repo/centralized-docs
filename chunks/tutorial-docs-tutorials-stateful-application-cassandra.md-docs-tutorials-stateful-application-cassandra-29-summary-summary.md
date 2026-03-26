---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#29-summary
chunk_level: summary
chunk_type: prose
heading: Validating the Cassandra StatefulSet
token_count: 94
summary: ``` `kubectl get pods -l=\"app=cassandra\" ` ``` The response should be similar to: ``` `NAME READY STATUS RESTARTS AGE cassandra-0 1/1 Running 0 1m cassandra-1 0/1 ContainerCreating 0 8s ` ``` It can...
---

```
`kubectl get pods -l="app=cassandra"
`
```
The response should be similar to:
```
`NAME READY STATUS RESTARTS AGE
cassandra-0 1/1 Running 0 1m
cassandra-1 0/1 ContainerCreating 0 8s
`
```
It can take several minutes for all three Pods to deploy. Once they are deployed, the same command
returns output similar to: