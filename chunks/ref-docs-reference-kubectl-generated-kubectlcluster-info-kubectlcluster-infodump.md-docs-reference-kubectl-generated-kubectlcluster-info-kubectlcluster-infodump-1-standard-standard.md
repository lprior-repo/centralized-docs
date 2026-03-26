---
doc_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump
chunk_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump#1-standard
chunk_level: standard
chunk_type: prose
heading: Examples
token_count: 251
summary: # kubectl cluster-info dump Dump relevant information for debugging and diagnosis ## Synopsis Dump cluster information out suitable for debugging and diagnosing cluster problems. By default, dumps...
---

# kubectl cluster-info dump
Dump relevant information for debugging and diagnosis
## Synopsis
Dump cluster information out suitable for debugging and diagnosing cluster problems. By default, dumps everything to stdout. You can optionally specify a directory with --output-directory. If you specify a directory, Kubernetes will build a set of files in that directory. By default, only dumps things in the current namespace and 'kube-system' namespace, but you can switch to a different namespace with the --namespaces flag, or specify --all-namespaces to dump all namespaces.
The command also dumps the logs of all of the pods in the cluster; these logs are dumped into different directories based on namespace and pod name.
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