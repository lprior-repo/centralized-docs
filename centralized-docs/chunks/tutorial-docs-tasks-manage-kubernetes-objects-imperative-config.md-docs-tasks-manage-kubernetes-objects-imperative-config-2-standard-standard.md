---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#2-standard
chunk_level: standard
chunk_type: prose
heading: How to view an object
token_count: 487
summary: ## Trade-offs The `kubectl` tool supports three kinds of object management: * Imperative commands * Imperative object configuration * Declarative object configuration See [Kubernetes Object...
---

## Trade-offs
The `kubectl` tool supports three kinds of object management:
* Imperative commands
* Imperative object configuration
* Declarative object configuration
See [Kubernetes Object Management](/docs/concepts/overview/working-with-objects/object-management/)
for a discussion of the advantages and disadvantage of each kind of object management.
## How to create objects
You can use `kubectl create -f` to create an object from a configuration file.
Refer to the [kubernetes API reference](/docs/reference/generated/kubernetes-api/v1.35/)
for details.
* `kubectl create -f &lt;filename|url&gt;`## How to update objects
#### Warning:
Updating objects with the `replace` command drops all
parts of the spec not specified in the configuration file. This
should not be used with objects whose specs are partially managed
by the cluster, such as Services of type `LoadBalancer`, where
the `externalIPs` field is managed independently from the configuration
file. Independently managed fields must be copied to the configuration
file to prevent `replace` from dropping them.
You can use `kubectl replace -f` to update a live object according to a
configuration file.
* `kubectl replace -f &lt;filename|url&gt;`## How to delete objects
You can use `kubectl delete -f` to delete an object that is described in a
configuration file.
* `kubectl delete -f &lt;filename|url&gt;`
#### Note:
If configuration file has specified the `generateName` field in the `metadata`
section instead of the `name` field, you cannot delete the object using
`kubectl delete -f &lt;filename|url&gt;`.
You will have to use other flags for deleting the object. For example:
```
`kubectl delete &lt;type&gt; &lt;name&gt;
kubectl delete &lt;type&gt; -l &lt;label&gt;
`
```
## How to view an object
You can use `kubectl get -f` to view information about an object that is
described in a configuration file.
* `kubectl get -f &lt;filename|url&gt; -o yaml`
The `-o yaml` flag specifies that the full object configuration is printed.
Use `kubectl get -h` to see a list of options.