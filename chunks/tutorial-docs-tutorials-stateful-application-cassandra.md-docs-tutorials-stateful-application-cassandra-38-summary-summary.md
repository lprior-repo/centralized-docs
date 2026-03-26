---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#38-summary
chunk_level: summary
chunk_type: prose
heading: Modifying the Cassandra StatefulSet
token_count: 123
summary: uid: 7a219483-6185-11e6-a910-42010a8a0fc0 spec: replicas: 3 ` ``` 2. Change the number of replicas to 4, and then save the manifest. The StatefulSet now scales to run with 4 Pods. 3. Get the...
---

uid: 7a219483-6185-11e6-a910-42010a8a0fc0
spec:
replicas: 3
`
```
2. Change the number of replicas to 4, and then save the manifest.
The StatefulSet now scales to run with 4 Pods.
3. Get the Cassandra StatefulSet to verify your change:
```
`kubectl get statefulset cassandra
`
```
The response should be similar to:
```
`NAME DESIRED CURRENT AGE
cassandra 4 4 36m
`
```