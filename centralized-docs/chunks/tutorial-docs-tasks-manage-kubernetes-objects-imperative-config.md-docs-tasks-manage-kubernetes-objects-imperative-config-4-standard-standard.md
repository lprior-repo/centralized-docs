---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#4-standard
chunk_level: standard
chunk_type: prose
heading: What's next
token_count: 395
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
```
`kubectl replace -f &lt;kind&gt;\_&lt;name&gt;.yaml
`
```
#### Warning:
Updating selectors on controllers is strongly discouraged.
The recommended approach is to define a single, immutable PodTemplate label
used only by the controller selector with no other semantic meaning.
Example label:
```
`selector:
matchLabels:
controller-selector: "apps/v1/deployment/nginx"
template:
metadata:
labels:
controller-selector: "apps/v1/deployment/nginx"
`
```
## What's next
* [Managing Kubernetes Objects Using Imperative Commands](/docs/tasks/manage-kubernetes-objects/imperative-command/)
* [Declarative Management of Kubernetes Objects Using Configuration Files](/docs/tasks/manage-kubernetes-objects/declarative-config/)
* [Kubectl Command Reference](/docs/reference/generated/kubectl/kubectl-commands/)
* [Kubernetes API Reference](/docs/reference/generated/kubernetes-api/v1.35/)