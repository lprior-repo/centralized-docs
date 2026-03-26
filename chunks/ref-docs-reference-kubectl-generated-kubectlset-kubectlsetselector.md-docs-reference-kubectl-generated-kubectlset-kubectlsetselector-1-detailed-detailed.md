---
doc_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector
chunk_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector#1-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 636
summary: # kubectl set selector Set the selector on a resource ## Synopsis Set the selector on a resource. Note that the new selector will overwrite the old selector if the resource had one prior to the...
---

# kubectl set selector
Set the selector on a resource
## Synopsis
Set the selector on a resource. Note that the new selector will overwrite the old selector if the resource had one prior to the invocation of 'set selector'.
A selector must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 63 characters. If --resource-version is specified, then updates will use this resource version, otherwise the existing resource-version will be used. Note: currently selectors can only be set on Service objects.
```
`kubectl set selector (-f FILENAME | TYPE NAME) EXPRESSIONS [--resource-version=version]
`
```
## Examples
```
` # Set the labels and selector before creating a deployment/service pair
kubectl create service clusterip my-svc --clusterip="None" -o yaml --dry-run=client | kubectl set selector --local -f - 'environment=qa' -o yaml | kubectl create -f -
kubectl create deployment my-dep --image=nginx -o yaml --dry-run=client | kubectl label --local -f - environment=qa -o yaml | kubectl create -f -
`
```
## Options
|--all|
||
Select all resources in the namespace of the specified resource types
|
|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|--dry-run string[="unchanged"]Default: "none"|
||
Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without sending it. If server strategy, submit server-side request without persisting the resource.
|
|--field-manager stringDefault: "kubectl-set"|
||
Name of the manager used to track field ownership.
|
|-f, --filename strings|
||
identifying the resource.
|
|-h, --help|
||
help for selector
|
|--local|
||
If true, annotation will NOT contact api-server but run locally.
|
|-o, --output string|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|-R, --recursiveDefault: true|
||
Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory.
|
|--resource-version string|
||
If non-empty, the selectors update will only succeed if this is the current resource-version for the object. Only valid when specifying a single resource.
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [http://golang.org/pkg/text/template/#pkg-overview].
|