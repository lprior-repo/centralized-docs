---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#6-detailed
chunk_level: detailed
chunk_type: code
heading: Related Pages
token_count: 963
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
## What's next
* Learn about [kubectl](/docs/concepts/overview/kubectl/) and its role in the Kubernetes ecosystem.
* [Install Minikube](https://minikube.sigs.k8s.io/docs/start/)
* See the [getting started guides](/docs/setup/) for more about creating clusters.
* [Learn how to launch and expose your application.](/docs/tasks/access-application-cluster/service-access-application-cluster/)
* If you need access to a cluster you didn't create, see the
[Sharing Cluster Access document](/docs/tasks/access-application-cluster/configure-access-multiple-clusters/).
* Read the [kubectl reference docs](/docs/reference/kubectl/kubectl/)
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified May 15, 2025 at 9:23 AM PST: [Add kuberc dedicated page (edac5dbf0e)](https://github.com/kubernetes/website/commit/edac5dbf0e2cf8c1ab7b8a3ee9daec7c54db7fab)
## Related Pages

- [install kubectl macos](docs-tasks-tools-install-kubectl-macos.md)
- [Access Applications in a Cluster](docs-tasks-access-application-cluster.md)
- [Monitoring, Logging, and Debugging](docs-tasks-debug.md)
- [Upgrading kubeadm clusters](docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md)
- [Auditing](docs-tasks-debug-debug-cluster-audit.md)