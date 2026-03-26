---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#9-standard
chunk_level: standard
chunk_type: code
heading: Validating the Cassandra StatefulSet
token_count: 308
summary: ## Validating the Cassandra StatefulSet 1. Get the Cassandra StatefulSet: ``` `kubectl get statefulset cassandra ` ``` The response should be similar to: ``` `NAME DESIRED CURRENT AGE cassandra 3 0...
---

## Validating the Cassandra StatefulSet
1. Get the Cassandra StatefulSet:
```
`kubectl get statefulset cassandra
`
```
The response should be similar to:
```
`NAME DESIRED CURRENT AGE
cassandra 3 0 13s
`
```
The `StatefulSet` resource deploys Pods sequentially.
2. Get the Pods to see the ordered creation status:
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
```
`NAME READY STATUS RESTARTS AGE
cassandra-0 1/1 Running 0 10m
cassandra-1 1/1 Running 0 9m
cassandra-2 1/1 Running 0 8m
`
```
3. Run the Cassandra [nodetool](https://cwiki.apache.org/confluence/display/CASSANDRA2/NodeTool) inside the first Pod, to
display the status of the ring.
```
`kubectl exec -it cassandra-0 -- nodetool status
`
```
The response should look something like: