---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#16-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 104
summary: If true, create a ClusterIP service associated with the pod. Requires --port. | |--field-manager stringDefault: \"kubectl-run\"| || Name of the manager used to track field ownership. | |-f, --filename...
---

If true, create a ClusterIP service associated with the pod. Requires --port.
|
|--field-manager stringDefault: "kubectl-run"|
||
Name of the manager used to track field ownership.
|
|-f, --filename strings|
||
to use to replace the resource.
|
|--force|
||
If true, immediately remove resources from API and bypass graceful deletion. Note that immediate deletion of some resources may result in inconsistency or data loss and requires confirmation.
|
|--grace-period intDefault: -1|
||