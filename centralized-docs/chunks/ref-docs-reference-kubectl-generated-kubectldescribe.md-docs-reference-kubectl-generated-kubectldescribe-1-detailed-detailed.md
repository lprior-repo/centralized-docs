---
doc_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe
chunk_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe#1-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 525
summary: # kubectl describe Show details of a specific resource or group of resources ## Synopsis Show details of a specific resource or group of resources. Print a detailed description of the selected...
---

# kubectl describe
Show details of a specific resource or group of resources
## Synopsis
Show details of a specific resource or group of resources.
Print a detailed description of the selected resources, including related resources such as events or controllers. You may select a single object by name, all objects of that type, provide a name prefix, or label selector. For example:
```
` $ kubectl describe TYPE NAME\_PREFIX
`
```
will first check for an exact match on TYPE and NAME\_PREFIX. If no such resource exists, it will output details for every resource that has a name prefixed with NAME\_PREFIX.
Use "kubectl api-resources" for a complete list of supported resources.
```
`kubectl describe (-f FILENAME | TYPE [NAME\_PREFIX | -l label] | TYPE/NAME)
`
```
## Examples
```
` # Describe a node
kubectl describe nodes kubernetes-node-emt8.c.myproject.internal
# Describe a pod identified by type and name in "pod.json"
kubectl describe -f pod.json
# Describe pods by label name=myLabel
kubectl describe pods -l name=myLabel
# Describe all pods managed by the 'frontend' replication controller
# (rc-created pods get the name of the rc as a prefix in the pod name)
kubectl describe pods frontend
`
```
## Options
|-A, --all-namespaces|
||
If present, list the requested object(s) across all namespaces. Namespace in current context is ignored even if specified with --namespace.
|
|--chunk-size intDefault: 500|
||
Return large lists in chunks rather than all at once. Pass 0 to disable.
|
|-f, --filename strings|
||
Filename, directory, or URL to files containing the resource to describe
|
|-h, --help|
||
help for describe
|
|-k, --kustomize string|
||
Process the kustomization directory. This flag can't be used together with -f or -R.
|
|-R, --recursive|
||
Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory.
|
|-l, --selector string|
||
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label constraints.
|
|--show-eventsDefault: true|
||
If true, display events related to the described object.
|