---
doc_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra
chunk_id: tutorial/docs-tutorials-stateful-application-cassandra.md/docs-tutorials-stateful-application-cassandra#35-summary
chunk_level: summary
chunk_type: prose
heading: Modifying the Cassandra StatefulSet
token_count: 82
summary: ## Modifying the Cassandra StatefulSet Use `kubectl edit` to modify the size of a Cassandra StatefulSet. 1. Run the following command: ``` `kubectl edit statefulset cassandra ` ``` This command opens...
---

## Modifying the Cassandra StatefulSet
Use `kubectl edit` to modify the size of a Cassandra StatefulSet.
1. Run the following command:
```
`kubectl edit statefulset cassandra
`
```
This command opens an editor in your terminal. The line you need to change is the `replicas` field.
The following sample is an excerpt of the StatefulSet file: