---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#15-summary
chunk_level: summary
chunk_type: prose
heading: Creating and editing an object from a URL without saving the configuration
token_count: 90
summary: ## Creating and editing an object from a URL without saving the configuration Suppose you have the URL of an object configuration file. You can use `kubectl create --edit` to make changes to the...
---

## Creating and editing an object from a URL without saving the configuration
Suppose you have the URL of an object configuration file. You can use
`kubectl create --edit` to make changes to the configuration before the
object is created. This is particularly useful for tutorials and tasks
that point to a configuration file that could be modified by the reader.
```
`kubectl create -f &lt;url&gt; --edit
`
```