---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#4-detailed
chunk_level: detailed
chunk_type: code
heading: What's next
token_count: 882
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
### Uninstall kubectl using homebrew
If you installed `kubectl` using Homebrew, run the following command:
```
`brew remove kubectl
`
```
## What's next
* Learn about [kubectl](/docs/concepts/overview/kubectl/) and its role in the Kubernetes ecosystem.
* [Install Minikube](https://minikube.sigs.k8s.io/docs/start/)
* See the [getting started guides](/docs/setup/) for more about creating clusters.
* [Learn how to launch and expose your application.](/docs/tasks/access-application-cluster/service-access-application-cluster/)
* If you need access to a cluster you didn't create, see the
[Sharing Cluster Access document](/docs/tasks/access-application-cluster/configure-access-multiple-clusters/).
* Read the [kubectl reference docs](/docs/reference/kubectl/kubectl/)