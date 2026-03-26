---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#11-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 112
summary: | |--dry-run string[=\"unchanged\"]Default: \"none\"| || Must be \"none\", \"server\", or \"client\". If client strategy, only print the object that would be sent, without sending it. If server strategy,...
---

|
|--dry-run string[="unchanged"]Default: "none"|
||
Must be "none", "server", or "client". If client strategy, only print the object that would be sent, without sending it. If server strategy, submit server-side request without persisting the resource.
|
|--field-manager stringDefault: "kubectl-label"|
||
Name of the manager used to track field ownership.
|
|--field-selector string|
||
Selector (field query) to filter on, supports '=', '==', and '!='.(e.g. --