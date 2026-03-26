---
doc_id: tutorial/pt-br-docs-concepts.md/pt-br-docs-concepts
chunk_id: tutorial/pt-br-docs-concepts.md/pt-br-docs-concepts#9-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 112
summary: ##### [Escalonamento, preempção e remoção](/pt-br/docs/concepts/scheduling-eviction/) No Kubernetes, escalonamento refere-se à certeza de que os Pods correspondam aos nós para que o kubelet possa...
---

##### [Escalonamento, preempção e remoção](/pt-br/docs/concepts/scheduling-eviction/)
No Kubernetes, escalonamento refere-se à certeza de que os Pods correspondam aos nós para que o kubelet possa executá-los. Preempção é o processo de finalizar Pods com menor prioridade, para que os Pods com maior prioridade possam ser escalonados nos nós. Remoção é o processo de finalização proativa de um ou mais Pods em nós com poucos recursos.