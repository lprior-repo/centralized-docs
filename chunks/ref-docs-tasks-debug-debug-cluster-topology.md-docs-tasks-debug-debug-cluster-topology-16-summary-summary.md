---
doc_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology
chunk_id: ref/docs-tasks-debug-debug-cluster-topology.md/docs-tasks-debug-debug-cluster-topology#16-summary
chunk_level: summary
chunk_type: prose
heading: Examine system logs
token_count: 120
summary: \"free\":103739236352 } }, \"nodes\":[ 0, 1 ] } }, \"entries\":{ \"fa9bdd38-6df9-4cf9-aa67-8c4814da37a8\":{ \"guaranteed\":[ { \"numaAffinity\":[ 0, 1 ], \"type\":\"memory\", \"size\":161061273600 } ] } },...
---

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