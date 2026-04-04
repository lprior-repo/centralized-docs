---
doc_id: ref/docs-reference-kubectl-generated-kubectltop-kubectltoppod.md/docs-reference-kubectl-generated-kubectltop-kubectltoppod
chunk_id: ref/docs-reference-kubectl-generated-kubectltop-kubectltoppod.md/docs-reference-kubectl-generated-kubectltop-kubectltoppod#1-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 445
summary: # kubectl top pod Display resource (CPU/memory) usage of pods ## Synopsis Display resource (CPU/memory) usage of pods. The 'top pod' command allows you to see the resource consumption of pods. Due to...
---

# kubectl top pod
Display resource (CPU/memory) usage of pods
## Synopsis
Display resource (CPU/memory) usage of pods.
The 'top pod' command allows you to see the resource consumption of pods.
Due to the metrics pipeline delay, they may be unavailable for a few minutes since pod creation.
```
`kubectl top pod [NAME | -l label]
`
```
## Examples
```
` # Show metrics for all pods in the default namespace
kubectl top pod
# Show metrics for all pods in the given namespace
kubectl top pod --namespace=NAMESPACE
# Show metrics for a given pod and its containers
kubectl top pod POD\_NAME --containers
# Show metrics for the pods defined by label name=myLabel
kubectl top pod -l name=myLabel
`
```
## Options
|-A, --all-namespaces|
||
If present, list the requested object(s) across all namespaces. Namespace in current context is ignored even if specified with --namespace.
|
|--containers|
||
If present, print usage of containers within a pod.
|
|--field-selector string|
||
Selector (field query) to filter on, supports '=', '==', and '!='.(e.g. --field-selector key1=value1,key2=value2). The server only supports a limited number of field queries per type.
|
|-h, --help|
||
help for pod
|
|--no-headers|
||
If present, print output without headers.
|
|-l, --selector string|
||
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label constraints.
|
|--show-swap|
||
Print pod resources related to swap memory.
|
|--sort-by string|
||
If non-empty, sort pods list using specified field. The field can be either 'cpu' or 'memory'.
|
|--sum|
||
Print the sum of the resource usage
|
|--use-protocol-buffersDefault: true|
||
Enables using protocol-buffers to access Metrics API.
|