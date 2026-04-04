---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#11-standard
chunk_level: standard
chunk_type: code
heading: Verify kubectl configuration
token_count: 402
summary: ### Install `kubectl convert` plugin A plugin for Kubernetes command-line tool `kubectl`, which allows you to convert manifests between different API versions. This can be particularly helpful to...
---

### Install `kubectl convert` plugin
A plugin for Kubernetes command-line tool `kubectl`, which allows you to convert manifests between different API
versions. This can be particularly helpful to migrate manifests to a non-deprecated api version with newer Kubernetes release.
For more info, visit [migrate to non deprecated apis](/docs/reference/using-api/deprecation-guide/#migrate-to-non-deprecated-apis)
1. Download the latest release with the command:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl-convert"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/arm64/kubectl-convert"
`
```
2. Validate the binary (optional)
Download the kubectl-convert checksum file:
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl-convert.sha256"
`
```
```
`
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/arm64/kubectl-convert.sha256"
`
```
Validate the kubectl-convert binary against the checksum file:
```
`echo "$(cat kubectl-convert.sha256) kubectl-convert" | sha256sum --check
`
```
If valid, the output is:
```
`kubectl-convert: OK
`
```
If the check fails, `sha256` exits with nonzero status and prints output similar to:
```
`kubectl-convert: FAILED
sha256sum: WARNING: 1 computed checksum did NOT match
`
```