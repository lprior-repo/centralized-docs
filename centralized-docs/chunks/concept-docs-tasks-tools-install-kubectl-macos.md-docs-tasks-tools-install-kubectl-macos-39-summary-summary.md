---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#39-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 106
summary: #### Note: Make sure `/usr/local/bin` is in your PATH environment variable. 5. Verify plugin is successfully installed ``` `kubectl convert --help ` ``` If you do not see an error, it means the...
---

#### Note:
Make sure `/usr/local/bin` is in your PATH environment variable.
5. Verify plugin is successfully installed
```
`kubectl convert --help
`
```
If you do not see an error, it means the plugin is successfully installed.
6. After installing the plugin, clean up the installation files:
```
`rm kubectl-convert kubectl-convert.sha256
`
```
### Uninstall kubectl on macOS
Depending on how you installed `kubectl`, use one of the following methods.