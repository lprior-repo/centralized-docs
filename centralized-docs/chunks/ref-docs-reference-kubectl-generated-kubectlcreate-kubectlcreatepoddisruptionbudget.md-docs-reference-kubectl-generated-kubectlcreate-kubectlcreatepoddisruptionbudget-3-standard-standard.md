---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreatepoddisruptionbudget#3-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 499
summary: |--allow-missing-template-keysDefault: true| || If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats. |...
---

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
help for poddisruptionbudget
|
|--max-unavailable string|
||
The maximum number or percentage of unavailable pods this budget requires.
|
|--min-available string|
||
The minimum number or percentage of available pods this budget requires.
|
|-o, --output string|
||
Output format. One of: (json, yaml, kyaml, name, go-template, go-template-file, template, templatefile, jsonpath, jsonpath-as-json, jsonpath-file).
|
|--save-config|
||
If true, the configuration of current object will be saved in its annotation. Otherwise, the annotation will be unchanged. This flag is useful when you want to perform kubectl apply on this object in the future.
|
|--selector string|
||
A label selector to use for this budget. Only equality-based selector requirements are supported.
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
Must be one of: strict (or true), warn, ignore (or false). "true" or "strict" will use a schema to validate the input and fail the request if invalid. It will perform server side validation if ServerSideFieldValidation is enabled on the api-server, but will fall back to less reliable client-side validation if not. "warn" will warn about unknown or duplicate fields without blocking the request if server-side field validation is enabled on the API server, and behave as "ignore" otherwise. "false" or "ignore"