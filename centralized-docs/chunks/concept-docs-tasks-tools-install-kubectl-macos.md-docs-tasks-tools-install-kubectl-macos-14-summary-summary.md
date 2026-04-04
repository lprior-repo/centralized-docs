---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#14-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on macOS
token_count: 79
summary: #### Note: Download the same version of the binary and checksum. 3. Make the kubectl binary executable. ``` `chmod +x ./kubectl ` ``` 4. Move the kubectl binary to a file location on your system...
---

#### Note:
Download the same version of the binary and checksum.
3. Make the kubectl binary executable.
```
`chmod +x ./kubectl
`
```
4. Move the kubectl binary to a file location on your system `PATH`.
```
`sudo mv ./kubectl /usr/local/bin/kubectl
sudo chown root: /usr/local/bin/kubectl
`
```