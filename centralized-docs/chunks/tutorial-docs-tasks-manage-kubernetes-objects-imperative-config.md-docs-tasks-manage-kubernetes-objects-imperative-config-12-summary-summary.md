---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#12-summary
chunk_level: summary
chunk_type: prose
heading: How to view an object
token_count: 79
summary: ## How to view an object You can use `kubectl get -f` to view information about an object that is described in a configuration file. * `kubectl get -f &lt;filename|url&gt; -o yaml` The `-o yaml` flag...
---

## How to view an object
You can use `kubectl get -f` to view information about an object that is
described in a configuration file.
* `kubectl get -f &lt;filename|url&gt; -o yaml`
The `-o yaml` flag specifies that the full object configuration is printed.
Use `kubectl get -h` to see a list of options.