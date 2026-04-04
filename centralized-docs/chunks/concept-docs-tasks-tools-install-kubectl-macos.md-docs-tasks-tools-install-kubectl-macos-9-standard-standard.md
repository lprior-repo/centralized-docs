---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#9-standard
chunk_level: standard
chunk_type: code
heading: What's next
token_count: 466
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