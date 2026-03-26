---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#3-detailed
chunk_level: detailed
chunk_type: table
heading: Related Pages
token_count: 861
summary: 't timeout requests.| |-s, --server string| ||The address and port of the Kubernetes API server| |--skip-headers| ||If true, avoid header prefixes in the log messages| |--skip-log-headers| ||If true,...
---

't timeout requests.|
|-s, --server string|
||The address and port of the Kubernetes API server|
|--skip-headers|
||If true, avoid header prefixes in the log messages|
|--skip-log-headers|
||If true, avoid headers when opening log files|
|--stderrthreshold severityDefault: 2|
||logs at or above this threshold go to stderr|
|--tls-server-name string|
||Server name to use for server certificate validation. If it is not provided, the hostname used to contact the server is used|
|--token string|
||Bearer token for authentication to the API server|
|--user string|
||The name of the kubeconfig user to use|
|--username string|
||Username for basic authentication to the API server|
|-v, --v Level|
||number for the log level verbosity|
|--version version[=true]|
||Print version information and quit|
|--vmodule moduleSpec|
||comma-separated list of pattern=N settings for file-filtered logging|
|--warnings-as-errors|
||Treat warnings received from the server as errors and exit with a non-zero exit code|
## Environment variables
|KUBECONFIG|
||Path to the kubectl configuration ("kubeconfig") file. Default: "$HOME/.kube/config"|
|KUBECTL\_EXPLAIN\_OPENAPIV3|
||Toggles whether calls to `kubectl explain` use the new OpenAPIv3 data source available. OpenAPIV3 is enabled by default since Kubernetes 1.24.|
|KUBECTL\_ENABLE\_CMD\_SHADOW|
||When set to true, external plugins can be used as subcommands for builtin commands if subcommand does not exist. In alpha stage, this feature can only be used for create command(e.g. kubectl create networkpolicy).|
|KUBECTL\_PORT\_FORWARD\_WEBSOCKETS|
||When set to true, the kubectl port-forward command will attempt to stream using the websockets protocol. If the upgrade to websockets fails, the commands will fallback to use the current SPDY protocol.|
|KUBECTL\_REMOTE\_COMMAND\_WEBSOCKETS|
||When set to true, the kubectl exec, cp, and attach commands will attempt to stream using the websockets protocol. If the upgrade to websockets fails, the commands will fallback to use the current SPDY protocol.|
|KUBECTL\_KUBERC|
||When set to true, kuberc file is taken into account to define user specific preferences.|
|KUBECTL\_KYAML|
||When set to true, kubectl is capable of producing Kubernetes-specific dialect of YAML output format.|
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
Last modified October 22, 2025 at 1:56 PM PST: [Remove KUBECTL\_COMMAND\_HEADERS which is promoted to stable (a3a2c17baf)](https://github.com/kubernetes/website/commit/a3a2c17bafa76d9859ac6a8acf7637ec587a4fb0)
## Related Pages

- [Access Clusters Using the Kubernetes API](docs-tasks-administer-cluster-access-cluster-api.md)
- [deploy intro](docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md)
- [Hello Minikube](docs-tutorials-hello-minikube.md)
- [Creating a cluster with kubeadm](docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md)
- [kubectl](docs-reference-kubectl-generated-kubectl.md)