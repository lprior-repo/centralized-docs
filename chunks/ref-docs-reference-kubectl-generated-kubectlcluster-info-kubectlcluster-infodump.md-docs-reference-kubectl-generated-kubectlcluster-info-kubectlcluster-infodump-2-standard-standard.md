---
doc_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump
chunk_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump#2-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 404
summary: ## Examples ``` ` # Dump current cluster state to stdout kubectl cluster-info dump # Dump current cluster state to /path/to/cluster-state kubectl cluster-info dump...
---

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
## Options
|-A, --all-namespaces|
||
If true, dump all namespaces. If true, --namespaces is ignored.
|
|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|-h, --help|
||
help for dump
|
|--namespaces strings|
||
A comma separated list of namespaces to dump.
|
|-o, --output stringDefault: "json"|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--output-directory string|
||
Where to output the files. If empty or '-' uses stdout, otherwise creates a directory hierarchy in that directory
|
|--pod-running-timeout durationDefault: 20s|
||
The length of time (like 5s, 2m, or 3h, higher than zero) to wait until at least one pod is running
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [http://golang.org/pkg/text/template/#pkg-overview].
|