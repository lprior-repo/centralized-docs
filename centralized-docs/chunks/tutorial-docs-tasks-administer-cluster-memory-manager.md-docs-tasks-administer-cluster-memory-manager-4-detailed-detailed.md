---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Reserved memory configuration
token_count: 760
summary: ### Memory manager reserved memory syntax Here are some examples of how to set the `reservedMemory` configuration for the kubelet. ``` ` # Example 1 reservedMemory: - numaNode: 0 # NUMA node index...
---

### Memory manager reserved memory syntax
Here are some examples of how to set the `reservedMemory` configuration for the kubelet.
```
` # Example 1
reservedMemory:
- numaNode: 0 # NUMA node index
limits:
memory: "1Gi" # byte quantity
- numaNode: 1
limits:
memory: "2Gi" # byte quantity
`
```
```
` # Example 2
reservedMemory:
- numaNode: 0
limits:
"memory": "512Gi"
- numaNode: 1
limits:
"memory": "512Gi"
"hugepages-1Gi": "2Gi" # only relevant on Linux
`
```
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
#### Note:
The default hard eviction threshold is 100MiB, and **not** zero.
Remember to increase the quantity of memory that you reserve by setting `reservedMemory`
by that hard eviction threshold. Otherwise, the kubelet will not start Memory Manager and
display an error.
Here is an example of a correct configuration that uses `reservedMemory`:
```
` # this snippet relies on the default value of evictionHard
memoryManagerPolicy: Static
kubeReserved: { cpu: "4", memory: "4Gi" }
systemReserved: { cpu: "1", memory: "1Gi" }
reservedMemory:
- numaNode: 0
limits:
memory: "3Gi"
- numaNode: 1
limits:
memory: "2148Mi" # 3GiB minus 100MiB
`
```