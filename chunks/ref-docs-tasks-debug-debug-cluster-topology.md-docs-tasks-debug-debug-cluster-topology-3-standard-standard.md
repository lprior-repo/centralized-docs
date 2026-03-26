---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#3-standard
chunk_level: standard
chunk_type: prose
heading: Examine system logs
token_count: 477
summary: ### Examine the memory manager state on a node Let us first deploy a sample `Guaranteed` pod whose specification is as follows: ``` `apiVersion: v1 kind: Pod metadata: name: guaranteed spec:...
---

### Examine the memory manager state on a node
Let us first deploy a sample `Guaranteed` pod whose specification is as follows:
```
`apiVersion: v1
kind: Pod
metadata:
name: guaranteed
spec:
containers:
- name: guaranteed
image: consumer
imagePullPolicy: Never
resources:
limits:
cpu: "2"
memory: 150Gi
requests:
cpu: "2"
memory: 150Gi
command: ["sleep","infinity"]
`
```
Next, log into the node where it was deployed and examine the state file in
`/var/lib/kubelet/memory\_manager\_state`:
```
`{
"policyName":"Static",
"machineState":{
"0":{
"numberOfAssignments":1,
"memoryMap":{
"hugepages-1Gi":{
"total":0,
"systemReserved":0,
"allocatable":0,
"reserved":0,
"free":0
},
"memory":{
"total":134987354112,
"systemReserved":3221225472,
"allocatable":131766128640,
"reserved":131766128640,
"free":0
}
},
"nodes":[
0,
1
]
},
"1":{
"numberOfAssignments":1,
"memoryMap":{
"hugepages-1Gi":{
"total":0,
"systemReserved":0,
"allocatable":0,
"reserved":0,
"free":0
},
"memory":{
"total":135286722560,
"systemReserved":2252341248,
"allocatable":133034381312,
"reserved":29295144960,
"free":103739236352
}
},
"nodes":[
0,
1
]
}
},
"entries":{
"fa9bdd38-6df9-4cf9-aa67-8c4814da37a8":{
"guaranteed":[
{
"numaAffinity":[
0,
1
],
"type":"memory",
"size":161061273600
}
]
}
},
"checksum":4142013182
}
`
```
It can be deduced from the state file that the pod was pinned to both NUMA nodes, i.e.:
```
`"numaAffinity":[
0,
1
],
`
```