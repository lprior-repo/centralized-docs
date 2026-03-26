---
url: https://kubernetes.io/docs/reference/kubectl/kubectl/
title: kubectl
word_count: 825
filtered: true
elements_removed: 0
density_score: 0.92
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Options](#options)
  - [Environment variables](#environment-variables)
  - [Feedback](#feedback)

---

## Synopsis
kubectl controls the Kubernetes cluster manager.
Find more information in [Command line tool](/docs/reference/kubectl/) (`kubectl`).
```
`kubectl [flags]
`
```
## Options
|--add-dir-header|
||If true, adds the file directory to the header of the log messages|
|--alsologtostderr|
||log to standard error as well as files|
|--as string|
||Username to impersonate for the operation|
|--as-group stringArray|
||Group to impersonate for the operation, this flag can be repeated to specify multiple groups.|
|--azure-container-registry-config string|
||Path to the file containing Azure container registry configuration information.|
|--cache-dir stringDefault: "$HOME/.kube/cache"|
||Default cache directory|
|--certificate-authority string|
||Path to a cert file for the certificate authority|
|--client-certificate string|
||Path to a client certificate file for TLS|
|--client-key string|
||Path to a client key file for TLS|
|--cloud-provider-gce-l7lb-src-cidrs cidrsDefault: 130.211.0.0/22,35.191.0.0/16|
||CIDRs opened in GCE firewall for L7 LB traffic proxy &amp; health checks|
|--cloud-provider-gce-lb-src-cidrs cidrsDefault: 130.211.0.0/22,209.85.152.0/22,209.85.204.0/22,35.191.0.0/16|
||CIDRs opened in GCE firewall for L4 LB traffic proxy &amp; health checks|
|--cluster string|
||The name of the kubeconfig cluster to use|
|--context string|
||The name of the kubeconfig context to use|
|--default-not-ready-toleration-seconds intDefault: 300|
||Indicates the tolerationSeconds of the toleration for notReady:NoExecute that is added by default to every pod that does not already have such a toleration.|
|--default-unreachable-toleration-seconds intDefault: 300|
||Indicates the tolerationSeconds of the toleration for unreachable:NoExecute that is added by default to every pod that does not already have such a toleration.|
|-h, --help|
||help for kubectl|
|--insecure-skip-tls-verify|
||If true, the server's certificate will not be checked for validity. This will make your HTTPS connections insecure|
|--kubeconfig string|
||Path to the kubeconfig file to use for CLI requests.|
|--log-backtrace-at traceLocationDefault: :0|
||when logging hits line file:N, emit a stack trace|
|--log-dir string|
||If non-empty, write log files in this directory|
|--log-file string|
||If non-empty, use this log file|
|--log-file-max-size uintDefault: 1800|
||Defines the maximum size a log file can grow to. Unit is megabytes. If the value is 0, the maximum file size is unlimited.|
|--log-flush-frequency durationDefault: 5s|
||Maximum number of seconds between log flushes|
|--logtostderrDefault: true|
||log to standard error instead of files|
|--match-server-version|
||Require server version to match client version|
|-n, --namespace string|
||If present, the namespace scope for this CLI request|
|--one-output|
||If true, only write logs to their native severity level (vs also writing to each lower severity level)|
|--password string|
||Password for basic authentication to the API server|
|--profile stringDefault: "none"|
||Name of profile to capture. One of (none|cpu|heap|goroutine|threadcreate|block|mutex)|
|--profile-output stringDefault: "profile.pprof"|
||Name of the file to write the profile to|
|--request-timeout stringDefault: "0"|
||The length of time to wait before giving up on a single server request. Non-zero values should contain a corresponding time unit (e.g. 1s, 2m, 3h). A value of zero means don't timeout requests.|
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
