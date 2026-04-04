---
doc_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied
chunk_id: ref/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied.md/docs-reference-kubectl-generated-kubectlapply-kubectlapplyedit-last-applied#2-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 459
summary: ## Options |--allow-missing-template-keysDefault: true| || If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output...
---

## Options
|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|--field-manager stringDefault: "kubectl-client-side-apply"|
||
Name of the manager used to track field ownership.
|
|-f, --filename strings|
||
Filename, directory, or URL to files to use to edit the resource
|
|-h, --help|
||
help for edit-last-applied
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
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|--template string|
||
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [http://golang.org/pkg/text/template/#pkg-overview].
|
|--validate string[="strict"]Default: "strict"|
||
Must be one of: strict (or true), warn, ignore (or false). "true" or "strict" will use a schema to validate the input and fail the request if invalid. It will perform server side validation if ServerSideFieldValidation is enabled on the api-server, but will fall back to less reliable client-side validation if not. "warn" will warn about unknown or duplicate fields without blocking the request if server-side field validation is enabled on the API server, and behave as "ignore" otherwise. "false" or "ignore" will not perform any schema validation, silently dropping any unknown or duplicate fields.
|
|--windows-line-endings|
||
Defaults to the line ending native to your platform.
|