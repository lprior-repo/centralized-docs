---
doc_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config
chunk_id: tutorial/docs-tasks-manage-kubernetes-objects-imperative-config.md/docs-tasks-manage-kubernetes-objects-imperative-config#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 684
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
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified May 19, 2022 at 5:00 PM PST: [Updated references in different pages with the correct titles of the documents (f559518520)](https://github.com/kubernetes/website/commit/f55951852059c85ceca96369909b5dce0f43b51d)
## Related Pages

- [Objects In Kubernetes](docs-concepts-overview-working-with-objects.md)
- [Monitoring, Logging, and Debugging](docs-tasks-debug.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)