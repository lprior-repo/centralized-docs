---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#1-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 783
summary: ## Examples ``` ` # Update pod 'foo' with the label 'unhealthy' and the value 'true' kubectl label pods foo unhealthy=true # Update pod 'foo' with the label 'status' and the value 'unhealthy',...
---

## Examples
```
` # Update pod 'foo' with the label 'unhealthy' and the value 'true'
kubectl label pods foo unhealthy=true
# Update pod 'foo' with the label 'status' and the value 'unhealthy', overwriting any existing value
kubectl label --overwrite pods foo status=unhealthy
# Update all pods in the namespace
kubectl label pods --all status=unhealthy
# Update a pod identified by the type and name in "pod.json"
kubectl label -f pod.json status=unhealthy
# Update pod 'foo' only if the resource is unchanged from version 1
kubectl label pods foo status=unhealthy --resource-version=1
# Update pod 'foo' by removing a label named 'bar' if it exists
# Does not require the --overwrite flag
kubectl label pods foo bar-
`
```
## Options
|--all|
||
Select all resources, in the namespace of the specified resource types
|
|-A, --all-namespaces|
||
If true, check the specified action in all namespaces.
|
|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|--dry-run string[="unchanged"]Default: "none"|
||
Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without sending it. If server strategy, submit server-side request without persisting the resource.
|
|--field-manager stringDefault: "kubectl-label"|
||
Name of the manager used to track field ownership.
|
|--field-selector string|
||
Selector (field query) to filter on, supports '=', '==', and '!='.(e.g. --field-selector key1=value1,key2=value2). The server only supports a limited number of field queries per type.
|
|-f, --filename strings|
||
Filename, directory, or URL to files identifying the resource to update the labels
|
|-h, --help|
||
help for label
|
|-k, --kustomize string|
||
Process the kustomization directory. This flag can't be used together with -f or -R.
|
|--list|
||
If true, display the labels for a given resource.
|
|--local|
||
If true, label will NOT contact api-server but run locally.
|
|-o, --output string|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--overwrite|
||
If true, allow labels to be overwritten, otherwise reject label updates that overwrite existing labels.
|
|-R, --recursive|
||
Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory.
|
|--resource-version string|
||
If non-empty, the labels update will only succeed if this is the current resource-version for the object. Only valid when specifying a single resource.
|
|-l, --selector string|
||
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label constraints.
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [http://golang.org/pkg/text/template/#pkg-overview].
|