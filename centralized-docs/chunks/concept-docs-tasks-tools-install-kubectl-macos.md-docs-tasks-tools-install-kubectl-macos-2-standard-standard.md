---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#2-standard
chunk_level: standard
chunk_type: code
heading: Install kubectl on macOS
token_count: 423
summary: #### Note: To download a specific version, replace the `$(curl -L -s https://dl.k8s.io/release/stable.txt)` portion of the command with the specific version. For example, to download version 1.35.0...
---

#### Note:
To download a specific version, replace the `$(curl -L -s https://dl.k8s.io/release/stable.txt)`
portion of the command with the specific version.
For example, to download version 1.35.0 on Intel macOS, type:
```
`curl -LO "https://dl.k8s.io/release/v1.35.0/bin/darwin/amd64/kubectl"
`
```
And for macOS on Apple Silicon, type:
```
`curl -LO "https://dl.k8s.io/release/v1.35.0/bin/darwin/arm64/kubectl"
`
```
2. Validate the binary (optional)
Download the kubectl checksum file:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl.sha256"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/arm64/kubectl.sha256"
`
```
Validate the kubectl binary against the checksum file:
```
`echo "$(cat kubectl.sha256) kubectl" | shasum -a 256 --check
`
```
If valid, the output is:
```
`kubectl: OK
`
```
If the check fails, `shasum` exits with nonzero status and prints output similar to:
```
`kubectl: FAILED
shasum: WARNING: 1 computed checksum did NOT match
`
```
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