---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#13-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 128
summary: #### Note: If you do not have root access on the target system, you can still install kubectl to the `\~/.local/bin` directory: ``` `chmod +x kubectl mkdir -p \~/.local/bin mv ./kubectl...
---

#### Note:
If you do not have root access on the target system, you can still install
kubectl to the `\~/.local/bin` directory:
```
`chmod +x kubectl
mkdir -p \~/.local/bin
mv ./kubectl \~/.local/bin/kubectl
# and then append (or prepend) \~/.local/bin to $PATH
`
```
4. Test to ensure the version you installed is up-to-date:
```
`kubectl version --client
`
```
Or use this for detailed view of version:
```
`kubectl version --client --output=yaml
`
```