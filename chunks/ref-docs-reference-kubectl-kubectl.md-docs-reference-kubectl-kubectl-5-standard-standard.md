---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#5-standard
chunk_level: standard
chunk_type: table
heading: Environment variables
token_count: 323
summary: ## Environment variables |KUBECONFIG| ||Path to the kubectl configuration (\"kubeconfig\") file. Default: \"$HOME/.kube/config\"| |KUBECTL\_EXPLAIN\_OPENAPIV3| ||Toggles whether calls to `kubectl...
---

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