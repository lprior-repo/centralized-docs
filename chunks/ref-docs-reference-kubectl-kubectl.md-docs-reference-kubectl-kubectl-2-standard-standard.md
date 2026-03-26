---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#2-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 505
summary: |--add-dir-header| ||If true, adds the file directory to the header of the log messages| |--alsologtostderr| ||log to standard error as well as files| |--as string| ||Username to impersonate for the...
---

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