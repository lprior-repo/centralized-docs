---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#17-summary
chunk_level: summary
chunk_type: prose
heading: Migrating from imperative commands to imperative object configuration
token_count: 51
summary: 2. Manually remove the status field from the object configuration file. 3. For subsequent object management, use `replace` exclusively. ``` `kubectl replace -f &lt;kind&gt;\_&lt;name&gt;.yaml ` ```
---

2. Manually remove the status field from the object configuration file.
3. For subsequent object management, use `replace` exclusively.
```
`kubectl replace -f &lt;kind&gt;\_&lt;name&gt;.yaml
`
```