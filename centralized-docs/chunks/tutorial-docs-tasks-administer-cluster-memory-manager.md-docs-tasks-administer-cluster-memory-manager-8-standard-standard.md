---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#8-standard
chunk_level: standard
chunk_type: prose
heading: Reserved memory configuration
token_count: 440
summary: ### Constraints on NUMA memory reservation When you specify values for `reservedMemory`, this must be compatible with the `kubeReserved` and `systemReserved` values that are in effect, along with any...
---

### Constraints on NUMA memory reservation
When you specify values for `reservedMemory`, this must be compatible with the `kubeReserved`
and `systemReserved` values that are in effect, along with any `memory.available` setting
you make as part of `evictionHard`.
$$\\begin{equation\*}
\\sum\_{ \\textnormal{i} = 0}^{ \\textnormal{node count}} { \\textit{reservedMemory} [ \\textnormal{i} ]} = \\textit{kubeReserved} + \\textit{systemReserved} + \\textit{evictionHard} \\, \\boxed{\\textnormal{memory.available}}
\\end{equation\*}\\\\\\
\\text{where i is an index of a NUMA node}$$
If you do not follow the formula above, the Memory Manager will show an error on startup.
In other words, the example 1 (above) illustrates that for the conventional memory (`type=memory`),
Kubernetes reserves 3GiB in total; that is:
$$\\begin{equation\*}
\\sum\_{ \\textnormal{i} = 0}^{ \\textnormal{node count}} \\textit{reservedMemory}\_{ [ \\textnormal{i} ] } = \\underbrace{\\textit{reservedMemory} [ 0 ] + \\textit{reservedMemory} [ 1 ] }\_{\\textnormal{type=memory}}
= 1 \\textnormal{GiB} + 2 \\textnormal{GiB}
= 3 \\textnormal{GiB}
\\end{equation\*}\\\\\\
\\text{where i is an index of a NUMA node}$$
Some examples of kubelet configuration settings relevant to the node allocatable configuration:
```
` kubeReserved: { cpu: "500m", memory: "50Mi" } # half a CPU, 50MiB of memory
systemReserved: { cpu: "500m", memory: "256Mi" } # half a CPU, 256MiB of memory
`
```