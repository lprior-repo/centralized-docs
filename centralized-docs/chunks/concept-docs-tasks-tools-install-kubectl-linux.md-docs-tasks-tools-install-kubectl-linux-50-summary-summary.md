---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#50-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 118
summary: #### Note: Download the same version of the binary and checksum. 3. Install kubectl-convert ``` `sudo install -o root -g root -m 0755 kubectl-convert /usr/local/bin/kubectl-convert ` ``` 4. Verify...
---

#### Note:
Download the same version of the binary and checksum.
3. Install kubectl-convert
```
`sudo install -o root -g root -m 0755 kubectl-convert /usr/local/bin/kubectl-convert
`
```
4. Verify plugin is successfully installed
```
`kubectl convert --help
`
```
If you do not see an error, it means the plugin is successfully installed.
5. After installing the plugin, clean up the installation files:
```
`rm kubectl-convert kubectl-convert.sha256
`
```