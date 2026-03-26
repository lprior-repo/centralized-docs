---
doc_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump
chunk_id: ref/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump.md/docs-reference-kubectl-generated-kubectlcluster-info-kubectlcluster-infodump#9-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 122
summary: '-' uses stdout, otherwise creates a directory hierarchy in that directory | |--pod-running-timeout durationDefault: 20s| || The length of time (like 5s, 2m, or 3h, higher than zero) to wait until at...
---

'-' uses stdout, otherwise creates a directory hierarchy in that directory
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
Template string or path to template file to use when -o=go-template, -o=go-template-file. The template format is golang templates [