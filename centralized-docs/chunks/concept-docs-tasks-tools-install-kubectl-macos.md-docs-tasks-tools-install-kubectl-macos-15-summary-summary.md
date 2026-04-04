---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#15-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on macOS
token_count: 91
summary: #### Note: Make sure `/usr/local/bin` is in your PATH environment variable. 5. Test to ensure the version you installed is up-to-date: ``` `kubectl version --client ` ``` Or use this for detailed...
---

#### Note:
Make sure `/usr/local/bin` is in your PATH environment variable.
5. Test to ensure the version you installed is up-to-date:
```
`kubectl version --client
`
```
Or use this for detailed view of version:
```
`kubectl version --client --output=yaml
`
```
6. After installing and validating kubectl, delete the checksum file:
```
`rm kubectl.sha256
`
```