---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#22-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 113
summary: '--attach' or with '-i/--stdin'. | |--save-config| || If true, the configuration of current object will be saved in its annotation. Otherwise, the annotation will be unchanged. This flag is useful...
---

'--attach' or with '-i/--stdin'.
|
|--save-config|
||
If true, the configuration of current object will be saved in its annotation. Otherwise, the annotation will be unchanged. This flag is useful when you want to perform kubectl apply on this object in the future.
|
|--show-managed-fields|
||
If true, keep the managedFields when printing objects in JSON or YAML format.
|
|-i, --stdin|
||
Keep stdin open on the container in the pod, even if nothing is attached.
|
|--template string|
||