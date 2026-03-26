---
doc_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector
chunk_id: ref/docs-reference-kubectl-generated-kubectlset-kubectlsetselector.md/docs-reference-kubectl-generated-kubectlset-kubectlsetselector#5-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 127
summary: |--all| || Select all resources in the namespace of the specified resource types | |--allow-missing-template-keysDefault: true| || If true, ignore any errors in templates when a field or map key is...
---

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
|--field-manager stringDefault: "