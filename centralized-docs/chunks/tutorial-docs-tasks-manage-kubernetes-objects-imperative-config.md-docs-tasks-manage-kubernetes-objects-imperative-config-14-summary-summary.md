---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#14-summary
chunk_level: summary
chunk_type: prose
heading: Limitations
token_count: 73
summary: 1. You create an object from a configuration file. 2. Another source updates the object by changing some field. 3. You replace the object from the configuration file. Changes made by the other source...
---

1. You create an object from a configuration file.
2. Another source updates the object by changing some field.
3. You replace the object from the configuration file. Changes made by
the other source in step 2 are lost.
If you need to support multiple writers to the same object, you can use
`kubectl apply` to manage the object.