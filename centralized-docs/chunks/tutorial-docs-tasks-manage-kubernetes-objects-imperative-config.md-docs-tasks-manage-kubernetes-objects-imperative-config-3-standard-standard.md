---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#3-standard
chunk_level: standard
chunk_type: prose
heading: Creating and editing an object from a URL without saving the configuration
token_count: 343
summary: ## How to view an object You can use `kubectl get -f` to view information about an object that is described in a configuration file. * `kubectl get -f &lt;filename|url&gt; -o yaml` The `-o yaml` flag...
---

## How to view an object
You can use `kubectl get -f` to view information about an object that is
described in a configuration file.
* `kubectl get -f &lt;filename|url&gt; -o yaml`
The `-o yaml` flag specifies that the full object configuration is printed.
Use `kubectl get -h` to see a list of options.
## Limitations
The `create`, `replace`, and `delete` commands work well when each object's
configuration is fully defined and recorded in its configuration
file. However when a live object is updated, and the updates are not merged
into its configuration file, the updates will be lost the next time a `replace`
is executed. This can happen if a controller, such as
a HorizontalPodAutoscaler, makes updates directly to a live object. Here's
an example:
1. You create an object from a configuration file.
2. Another source updates the object by changing some field.
3. You replace the object from the configuration file. Changes made by
the other source in step 2 are lost.
If you need to support multiple writers to the same object, you can use
`kubectl apply` to manage the object.
## Creating and editing an object from a URL without saving the configuration
Suppose you have the URL of an object configuration file. You can use
`kubectl create --edit` to make changes to the configuration before the
object is created. This is particularly useful for tutorials and tasks
that point to a configuration file that could be modified by the reader.
```
`kubectl create -f &lt;url&gt; --edit
`
```