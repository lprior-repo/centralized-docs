---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#49-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 103
summary: Validate the kubectl-convert binary against the checksum file: ``` `echo \"$(cat kubectl-convert.sha256) kubectl-convert\" | sha256sum --check ` ``` If valid, the output is: ``` `kubectl-convert: OK `...
---

Validate the kubectl-convert binary against the checksum file:
```
`echo "$(cat kubectl-convert.sha256) kubectl-convert" | sha256sum --check
`
```
If valid, the output is:
```
`kubectl-convert: OK
`
```
If the check fails, `sha256` exits with nonzero status and prints output similar to:
```
`kubectl-convert: FAILED
sha256sum: WARNING: 1 computed checksum did NOT match
`
```