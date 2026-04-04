---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Verify kubectl configuration
token_count: 665
summary: ## Verify kubectl configuration In order for kubectl to find and access a Kubernetes cluster, it needs a [kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/), which is...
---

## Verify kubectl configuration
In order for kubectl to find and access a Kubernetes cluster, it needs a
[kubeconfig file](/docs/concepts/configuration/organize-cluster-access-kubeconfig/),
which is created automatically when you create a cluster using
[kube-up.sh](https://github.com/kubernetes/kubernetes/blob/master/cluster/kube-up.sh)
or successfully deploy a Minikube cluster.
By default, kubectl configuration is located at `\~/.kube/config`.
Check that kubectl is properly configured by getting the cluster state:
```
`kubectl cluster-info
`
```
If you see a URL response, kubectl is correctly configured to access your cluster.
If you see a message similar to the following, kubectl is not configured correctly
or is not able to connect to a Kubernetes cluster.
```
`The connection to the server &lt;server-name:port&gt; was refused - did you specify the right host or port?
`
```
For example, if you are intending to run a Kubernetes cluster on your laptop (locally),
you will need a tool like [Minikube](https://minikube.sigs.k8s.io/docs/start/) to be
installed first and then re-run the commands stated above.
If `kubectl cluster-info` returns the url response, but you can't access your cluster,
check whether it is configured properly using the following command:
```
`kubectl cluster-info dump
`
```
### Troubleshooting the 'No Auth Provider Found' error message
In Kubernetes 1.26, kubectl removed the built-in authentication for the following cloud
providers' managed Kubernetes offerings. These providers have released kubectl plugins
to provide the cloud-specific authentication. For instructions, refer to the following provider documentation:
* Azure AKS: [kubelogin plugin](https://azure.github.io/kubelogin/)
* Google Kubernetes Engine: [gke-gcloud-auth-plugin](https://cloud.google.com/kubernetes-engine/docs/how-to/cluster-access-for-kubectl#install_plugin)
There could also be other causes for the same error message that are unrelated
to that change.
### Introduction
The kubectl completion script for Bash can be generated with `kubectl completion bash`.
Sourcing this script in your shell enables kubectl completion.
However, the kubectl completion script depends on
[**bash-completion**](https://github.com/scop/bash-completion) which you thus have to previously install.
#### Warning:
There are two versions of bash-completion, v1 and v2. V1 is for Bash 3.2
(which is the default on macOS), and v2 is for Bash 4.1+. The kubectl completion
script **doesn't work** correctly with bash-completion v1 and Bash 3.2.
It requires **bash-completion v2** and **Bash 4.1+**. Thus, to be able to
correctly use kubectl completion on macOS, you have to install and use
Bash 4.1+ ([*instructions*](https://apple.stackexchange.com/a/292760)).
The following instructions assume that you use Bash 4.1+
(that is, any Bash version of 4.1 or newer).