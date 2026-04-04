---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#41-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 68
summary: If you have an alias for kubectl, you can extend shell completion to work with that alias: ``` `echo 'alias k=kubectl' &gt;&gt;\~/.bashrc echo 'complete -o default -F \_\_start\_kubectl k'...
---

If you have an alias for kubectl, you can extend shell completion to work with that alias:
```
`echo 'alias k=kubectl' &gt;&gt;\~/.bashrc
echo 'complete -o default -F \_\_start\_kubectl k' &gt;&gt;\~/.bashrc
`
```