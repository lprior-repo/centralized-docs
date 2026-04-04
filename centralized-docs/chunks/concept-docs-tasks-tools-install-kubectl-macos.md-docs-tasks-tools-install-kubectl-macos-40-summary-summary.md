---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#40-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 114
summary: ### Uninstall kubectl on macOS Depending on how you installed `kubectl`, use one of the following methods. ### Uninstall kubectl using the command-line 1. Locate the `kubectl` binary on your system:...
---

### Uninstall kubectl on macOS
Depending on how you installed `kubectl`, use one of the following methods.
### Uninstall kubectl using the command-line
1. Locate the `kubectl` binary on your system:
```
`which kubectl
`
```
2. Remove the `kubectl` binary:
```
`sudo rm &lt;path&gt;
`
```
Replace `&lt;path&gt;` with the path to the `kubectl` binary from the previous step. For example, `sudo rm /usr/local/bin/kubectl`.