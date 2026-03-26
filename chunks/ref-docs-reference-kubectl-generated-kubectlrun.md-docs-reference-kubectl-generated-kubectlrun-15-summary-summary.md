---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#15-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 121
summary: 'args' field which is the default. | |--dry-run string[=\"unchanged\"]Default: \"none\"| || Must be \"none\", \"server\", or \"client\". If client strategy, only print the object that would be sent, without...
---

'args' field which is the default.
|
|--dry-run string[="unchanged"]Default: "none"|
||
Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without sending it. If server strategy, submit server-side request without persisting the resource.
|
|--env strings|
||
Environment variables to set in the container.
|
|--expose --port|
||
If true, create a ClusterIP service associated with the pod. Requires --port.
|
|--field-manager stringDefault: "kubectl-run"|
||