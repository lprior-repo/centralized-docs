---
doc_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump
chunk_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump#5-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 112
summary: ``` `kubectl cluster-info dump [flags] ` ``` ## Examples ``` ` # Dump current cluster state to stdout kubectl cluster-info dump # Dump current cluster state to /path/to/cluster-state kubectl...
---

```
`kubectl cluster-info dump [flags]
`
```
## Examples
```
` # Dump current cluster state to stdout
kubectl cluster-info dump
# Dump current cluster state to /path/to/cluster-state
kubectl cluster-info dump --output-directory=/path/to/cluster-state
# Dump all namespaces to stdout
kubectl cluster-info dump --all-namespaces
# Dump a set of namespaces to /path/to/cluster-state
kubectl cluster-info dump --namespaces default,kube-system --output-directory=/path/to/cluster-state
`
```