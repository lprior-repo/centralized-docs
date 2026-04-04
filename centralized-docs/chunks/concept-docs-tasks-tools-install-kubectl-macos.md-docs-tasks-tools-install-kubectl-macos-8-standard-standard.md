---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#8-standard
chunk_level: standard
chunk_type: code
heading: Verify kubectl configuration
token_count: 506
summary: ### Install `kubectl convert` plugin A plugin for Kubernetes command-line tool `kubectl`, which allows you to convert manifests between different API versions. This can be particularly helpful to...
---

### Install `kubectl convert` plugin
A plugin for Kubernetes command-line tool `kubectl`, which allows you to convert manifests between different API
versions. This can be particularly helpful to migrate manifests to a non-deprecated api version with newer Kubernetes release.
For more info, visit [migrate to non deprecated apis](/docs/reference/using-api/deprecation-guide/#migrate-to-non-deprecated-apis)
1. Download the latest release with the command:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl-convert"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/arm64/kubectl-convert"
`
```
2. Validate the binary (optional)
Download the kubectl-convert checksum file:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl-convert.sha256"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/arm64/kubectl-convert.sha256"
`
```
Validate the kubectl-convert binary against the checksum file:
```
`echo "$(cat kubectl-convert.sha256) kubectl-convert" | shasum -a 256 --check
`
```
If valid, the output is:
```
`kubectl-convert: OK
`
```
If the check fails, `shasum` exits with nonzero status and prints output similar to:
```
`kubectl-convert: FAILED
shasum: WARNING: 1 computed checksum did NOT match
`
```
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