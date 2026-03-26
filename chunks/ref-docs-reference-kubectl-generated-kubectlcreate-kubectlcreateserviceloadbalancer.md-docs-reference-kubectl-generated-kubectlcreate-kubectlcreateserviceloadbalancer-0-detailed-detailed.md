---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateserviceloadbalancer#0-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 617
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Examples](#examples)   - [Options](#options)   - [Parent Options Inherited](#parent-options-inherited)   - [Feedback](#feedback)  ---  ## Synopsis...
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---

## Synopsis
Create a LoadBalancer service with the specified name.
```
`kubectl create service loadbalancer NAME [--tcp=port:targetPort] [--dry-run=server|client|none]
`
```
## Examples
```
` # Create a new LoadBalancer service named my-lbs
kubectl create service loadbalancer my-lbs --tcp=5678:8080
`
```
## Options
|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|--dry-run string[="unchanged"]Default: "none"|
||
Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without sending it. If server strategy, submit server-side request without persisting the resource.
|
|--field-manager stringDefault: "kubectl-create"|
||
Name of the manager used to track field ownership.
|
|-h, --help|
||
help for loadbalancer
|
|-o, --output string|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--save-config|
||
If true, the configuration of current object will be saved in its annotation. Otherwise, the annotation will be unchanged. This flag is useful when you want to perform kubectl apply on this object in the future.
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--tcp strings|
||
Port pairs can be specified as '&lt;port&gt;:&lt;targetPort&gt;'.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [http://golang.org/pkg/text/template/#pkg-overview].
|
|--validate string[="strict"]Default: "strict"|
||
Must be one of: strict (or true), warn, ignore (or false). "true" or "strict" will use a schema to validate the input and fail the request if invalid. It will perform server side validation if ServerSideFieldValidation is enabled on the api-server, but will fall back to less reliable client-side validation if not. "warn" will warn about unknown or duplicate fields without blocking the request if server-side field validation is enabled on the API server, and behave as "ignore" otherwise. "false" or "ignore" will not perform any schema validation, silently dropping any unknown or duplicate fields.
|