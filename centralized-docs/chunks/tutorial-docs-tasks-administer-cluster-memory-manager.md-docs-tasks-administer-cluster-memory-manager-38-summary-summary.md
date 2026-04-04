---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#38-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 115
summary: = 3 \\textnormal{GiB} \\end{equation\*}\\\\\\ \\text{where i is an index of a NUMA node}$$ Some examples of kubelet configuration settings relevant to the node allocatable configuration: ``` `...
---

= 3 \\textnormal{GiB}
\\end{equation\*}\\\\\\
\\text{where i is an index of a NUMA node}$$
Some examples of kubelet configuration settings relevant to the node allocatable configuration:
```
` kubeReserved: { cpu: "500m", memory: "50Mi" } # half a CPU, 50MiB of memory
systemReserved: { cpu: "500m", memory: "256Mi" } # half a CPU, 256MiB of memory
`
```