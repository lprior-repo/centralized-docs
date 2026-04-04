---
doc_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager
chunk_id: tutorial/docs-tasks-administer-cluster-memory-manager.md/docs-tasks-administer-cluster-memory-manager#37-summary
chunk_level: summary
chunk_type: prose
heading: Reserved memory configuration
token_count: 124
summary: $$\\begin{equation\*} \\sum\_{ \\textnormal{i} = 0}^{ \\textnormal{node count}} \\textit{reservedMemory}\_{ [ \\textnormal{i} ] } = \\underbrace{\\textit{reservedMemory} [ 0 ] +...
---

$$\\begin{equation\*}
\\sum\_{ \\textnormal{i} = 0}^{ \\textnormal{node count}} \\textit{reservedMemory}\_{ [ \\textnormal{i} ] } = \\underbrace{\\textit{reservedMemory} [ 0 ] + \\textit{reservedMemory} [ 1 ] }\_{\\textnormal{type=memory}}
= 1 \\textnormal{GiB} + 2 \\textnormal{GiB}
= 3 \\textnormal{GiB}
\\end{equation\*}\\\\\\