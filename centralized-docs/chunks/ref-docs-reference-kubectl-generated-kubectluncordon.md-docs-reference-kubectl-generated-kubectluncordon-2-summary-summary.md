---
doc_id: ref/docs-reference-kubectl-generated-kubectluncordon.md/docs-reference-kubectl-generated-kubectluncordon
chunk_id: ref/docs-reference-kubectl-generated-kubectluncordon.md/docs-reference-kubectl-generated-kubectluncordon#2-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 104
summary: |--dry-run string[=\"unchanged\"]Default: \"none\"| || Must be \"none\", \"server\", or \"client\". If client strategy, only print the object that would be sent, without sending it. If server strategy, submit...
---

|--dry-run string[="unchanged"]Default: "none"|
||
Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without sending it. If server strategy, submit server-side request without persisting the resource.
|
|-h, --help|
||
help for uncordon
|
|-l, --selector string|
||
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'