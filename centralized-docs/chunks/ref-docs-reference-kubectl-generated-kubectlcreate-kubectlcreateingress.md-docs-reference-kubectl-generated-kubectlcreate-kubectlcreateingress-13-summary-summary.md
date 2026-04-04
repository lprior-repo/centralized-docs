---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#13-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 124
summary: |--allow-missing-template-keysDefault: true| || If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats. |...
---

|--allow-missing-template-keysDefault: true|
||
If true, ignore any errors in templates when a field or map key is missing in the template. Only applies to golang and jsonpath output formats.
|
|--annotation strings|
||
Annotation to insert in the ingress object, in the format annotation=value
|
|--class string|
||
Ingress Class to be used
|
|--default-backend string|
||
Default service for backend, in format of svcname:port
|
|--dry-run string[="unchanged"]Default: "none"|
||
Must be "none", "server", or "client"