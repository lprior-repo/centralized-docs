---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#2-standard
chunk_level: standard
chunk_type: prose
heading: Install kubectl on Linux
token_count: 175
summary: #### Note: Download the same version of the binary and checksum. 3. Install kubectl ``` `sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl ` ``` #### Note: If you do not have root...
---

#### Note:
Download the same version of the binary and checksum.
3. Install kubectl
```
`sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
`
```
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