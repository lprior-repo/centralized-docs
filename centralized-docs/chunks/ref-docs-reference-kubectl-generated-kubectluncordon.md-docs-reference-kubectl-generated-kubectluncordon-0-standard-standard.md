---
doc_id: ref/docs-reference-kubectl-generated-kubectluncordon.md/docs-reference-kubectl-generated-kubectluncordon
chunk_id: ref/docs-reference-kubectl-generated-kubectluncordon.md/docs-reference-kubectl-generated-kubectluncordon#0-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 239
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Examples](#examples)   - [Options](#options)   - [Parent Options Inherited](#parent-options-inherited)   - [Feedback](#feedback)  ---  ## Synopsis...
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---

## Synopsis
Mark node as schedulable.
```
`kubectl uncordon NODE
`
```
## Examples
```
` # Mark node "foo" as schedulable
kubectl uncordon foo
`
```
## Options
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
Selector (label query) to filter on, supports '=', '==', '!=', 'in', 'notin'.(e.g. -l key1=value1,key2=value2,key3 in (value3)). Matching objects must satisfy all of the specified label constraints.
|