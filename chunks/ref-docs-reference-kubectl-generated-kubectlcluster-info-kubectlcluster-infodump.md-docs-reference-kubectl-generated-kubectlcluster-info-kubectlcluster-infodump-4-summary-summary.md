---
doc_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump
chunk_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump#4-summary
chunk_level: summary
chunk_type: prose
heading: Synopsis
token_count: 124
summary: ## Synopsis Dump cluster information out suitable for debugging and diagnosing cluster problems. By default, dumps everything to stdout. You can optionally specify a directory with...
---

## Synopsis
Dump cluster information out suitable for debugging and diagnosing cluster problems. By default, dumps everything to stdout. You can optionally specify a directory with --output-directory. If you specify a directory, Kubernetes will build a set of files in that directory. By default, only dumps things in the current namespace and 'kube-system' namespace, but you can switch to a different namespace with the --namespaces flag, or specify --all-namespaces to dump all namespaces.
The command also dumps the logs of all of the pods in the cluster; these logs are dumped into different directories based on namespace and pod name.