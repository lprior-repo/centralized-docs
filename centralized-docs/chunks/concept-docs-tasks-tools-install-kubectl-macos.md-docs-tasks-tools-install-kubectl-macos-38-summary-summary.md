---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#38-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 90
summary: #### Note: Download the same version of the binary and checksum. 3. Make kubectl-convert binary executable ``` `chmod +x ./kubectl-convert ` ``` 4. Move the kubectl-convert binary to a file location...
---

#### Note:
Download the same version of the binary and checksum.
3. Make kubectl-convert binary executable
```
`chmod +x ./kubectl-convert
`
```
4. Move the kubectl-convert binary to a file location on your system `PATH`.
```
`sudo mv ./kubectl-convert /usr/local/bin/kubectl-convert
sudo chown root: /usr/local/bin/kubectl-convert
`
```