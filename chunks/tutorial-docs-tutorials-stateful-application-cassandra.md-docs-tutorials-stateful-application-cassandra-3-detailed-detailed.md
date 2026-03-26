---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#3-detailed
chunk_level: detailed
chunk_type: code
heading: Modifying the Cassandra StatefulSet
token_count: 823
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
```
`Datacenter: DC1-K8Demo
======================
Status=Up/Down
|/ State=Normal/Leaving/Joining/Moving
-- Address Load Tokens Owns (effective) Host ID Rack
UN 172.17.0.5 83.57 KiB 32 74.0% e2dd09e6-d9d3-477e-96c5-45094c08db0f Rack1-K8Demo
UN 172.17.0.4 101.04 KiB 32 58.8% f89d6835-3a42-4419-92b3-0e62cae1479c Rack1-K8Demo
UN 172.17.0.6 84.74 KiB 32 67.1% a6a1e8c2-3dc5-4417-b1a0-26507af2aaad Rack1-K8Demo
`
```
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