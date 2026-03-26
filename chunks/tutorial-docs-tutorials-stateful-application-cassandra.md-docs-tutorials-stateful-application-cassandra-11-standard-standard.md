---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#11-standard
chunk_level: standard
chunk_type: prose
heading: Modifying the Cassandra StatefulSet
token_count: 306
summary: ## Modifying the Cassandra StatefulSet Use `kubectl edit` to modify the size of a Cassandra StatefulSet. 1. Run the following command: ``` `kubectl edit statefulset cassandra ` ``` This command opens...
---

## Modifying the Cassandra StatefulSet
Use `kubectl edit` to modify the size of a Cassandra StatefulSet.
1. Run the following command:
```
`kubectl edit statefulset cassandra
`
```
This command opens an editor in your terminal. The line you need to change is the `replicas` field.
The following sample is an excerpt of the StatefulSet file:
```
`# Please edit the object below. Lines beginning with a '#' will be ignored,
# and an empty file will abort the edit. If an error occurs while saving this file will be
#
apiVersion: apps/v1
kind: StatefulSet
metadata:
creationTimestamp: 2016-08-13T18:40:58Z
generation: 1
labels:
app: cassandra
name: cassandra
namespace: default
resourceVersion: "323"
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