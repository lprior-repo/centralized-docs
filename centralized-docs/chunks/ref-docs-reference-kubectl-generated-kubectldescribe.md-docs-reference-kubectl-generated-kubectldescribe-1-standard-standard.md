---
doc_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe
chunk_id: ref/docs-reference-kubectl-generated-kubectldescribe.md/docs-reference-kubectl-generated-kubectldescribe#1-standard
chunk_level: standard
chunk_type: table
heading: Examples
token_count: 272
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