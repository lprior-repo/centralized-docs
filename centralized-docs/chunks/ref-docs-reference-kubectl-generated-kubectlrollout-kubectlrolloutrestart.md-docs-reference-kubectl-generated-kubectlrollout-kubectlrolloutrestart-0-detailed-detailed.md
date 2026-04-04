---
doc_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart
chunk_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutrestart#0-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 517
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Examples](#examples) - [Restart deployments with the app=nginx label](#restart-deployments-with-the-appnginx-label)   - [Options](#options)   -...
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
- [Restart deployments with the app=nginx label](#restart-deployments-with-the-appnginx-label)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---

## Synopsis
Restart a resource.
```
` Resource rollout will be restarted.
`
```
```
`kubectl rollout restart RESOURCE
`
```
## Examples
```
` # Restart all deployments in the test-namespace namespace
kubectl rollout restart deployment -n test-namespace
# Restart deployments with the app=nginx label
kubectl rollout restart deployment --selector=app=nginx
`
```
## Options
|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|--field-manager stringDefault: "kubectl-rollout"|
||
Name of the manager used to track field ownership.
|
|-f, --filename strings|
||
Filename, directory, or URL to files identifying the resource to get from a server.
|
|-h, --help|
||
help for restart
|
|-k, --kustomize string|
||
Process the kustomization directory. This flag can't be used together with -f or -R.
|
|-o, --output string|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|-R, --recursive|
||
Process the directory used in -f, --filename recursively. Useful when you want to manage related manifests organized within the same directory.
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