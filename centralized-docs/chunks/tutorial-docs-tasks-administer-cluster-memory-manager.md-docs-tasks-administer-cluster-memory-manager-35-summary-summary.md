---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#35-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 109
summary: $$\\begin{equation\*} \\sum\_{ \\textnormal{i} = 0}^{ \\textnormal{node count}} { \\textit{reservedMemory} [ \\textnormal{i} ]} = \\textit{kubeReserved} + \\textit{systemReserved} +...
---

$$\\begin{equation\*}
\\sum\_{ \\textnormal{i} = 0}^{ \\textnormal{node count}} { \\textit{reservedMemory} [ \\textnormal{i} ]} = \\textit{kubeReserved} + \\textit{systemReserved} + \\textit{evictionHard} \\, \\boxed{\\textnormal{memory.available}}
\\end{equation\*}\\\\\\
\\text{where i is an index of a NUMA node}$$