---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#14-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: '--restart=Never' the exit code of the container process is returned. | |--cascade string[=\"background\"]Default: \"background\"| || Must be \"background\", \"orphan\", or \"foreground\". Selects the deletion...
---

'--restart=Never' the exit code of the container process is returned.
|
|--cascade string[="background"]Default: "background"|
||
Must be "background", "orphan", or "foreground". Selects the deletion cascading strategy for the dependents (e.g. Pods created by a ReplicationController). Defaults to background.
|
|--command|
||
If true and extra arguments are present, use them as the 'command' field in the container, rather than the 'args' field which is the default.
|
|--dry-run string[="unchanged"]Default: "none"|
||
Must be "