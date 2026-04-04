---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#37-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 108
summary: Validate the kubectl-convert binary against the checksum file: ``` `echo \"$(cat kubectl-convert.sha256) kubectl-convert\" | shasum -a 256 --check ` ``` If valid, the output is: ``` `kubectl-convert:...
---

Validate the kubectl-convert binary against the checksum file:
```
`echo "$(cat kubectl-convert.sha256) kubectl-convert" | shasum -a 256 --check
`
```
If valid, the output is:
```
`kubectl-convert: OK
`
```
If the check fails, `shasum` exits with nonzero status and prints output similar to:
```
`kubectl-convert: FAILED
shasum: WARNING: 1 computed checksum did NOT match
`
```