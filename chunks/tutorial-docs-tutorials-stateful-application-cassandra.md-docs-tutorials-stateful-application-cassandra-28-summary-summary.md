---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#28-summary
chunk_level: summary
chunk_type: prose
heading: Validating the Cassandra StatefulSet
token_count: 109
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