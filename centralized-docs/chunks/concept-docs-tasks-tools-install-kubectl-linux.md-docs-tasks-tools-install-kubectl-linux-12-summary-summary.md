---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#12-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 99
summary: ``` `kubectl: OK ` ``` If the check fails, `sha256` exits with nonzero status and prints output similar to: ``` `kubectl: FAILED sha256sum: WARNING: 1 computed checksum did NOT match ` ``` #### Note:...
---

```
`kubectl: OK
`
```
If the check fails, `sha256` exits with nonzero status and prints output similar to:
```
`kubectl: FAILED
sha256sum: WARNING: 1 computed checksum did NOT match
`
```
#### Note:
Download the same version of the binary and checksum.
3. Install kubectl
```
`sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
`
```