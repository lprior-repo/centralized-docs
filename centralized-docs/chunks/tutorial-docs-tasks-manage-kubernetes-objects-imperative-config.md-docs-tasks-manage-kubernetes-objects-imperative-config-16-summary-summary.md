---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#16-summary
chunk_level: summary
chunk_type: prose
heading: Migrating from imperative commands to imperative object configuration
token_count: 110
summary: ## Migrating from imperative commands to imperative object configuration Migrating from imperative commands to imperative object configuration involves several manual steps. 1. Export the live object...
---

## Migrating from imperative commands to imperative object configuration
Migrating from imperative commands to imperative object configuration involves
several manual steps.
1. Export the live object to a local object configuration file:
```
`kubectl get &lt;kind&gt;/&lt;name&gt; -o yaml &gt; &lt;kind&gt;\_&lt;name&gt;.yaml
`
```
2. Manually remove the status field from the object configuration file.
3. For subsequent object management, use `replace` exclusively.