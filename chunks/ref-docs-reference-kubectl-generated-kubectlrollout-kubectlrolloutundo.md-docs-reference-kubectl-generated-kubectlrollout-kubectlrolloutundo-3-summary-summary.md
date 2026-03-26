---
doc_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo
chunk_id: ref/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo.md/docs-reference-kubectl-generated-kubectlrollout-kubectlrolloutundo#3-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
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
|-f, --filename strings|
||
Filename, directory, or URL to files identifying the resource to get from a server.