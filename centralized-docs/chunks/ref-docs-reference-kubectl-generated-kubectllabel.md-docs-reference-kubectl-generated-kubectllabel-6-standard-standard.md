---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#6-standard
chunk_level: standard
chunk_type: table
heading: Parent Options Inherited
token_count: 508
summary: |--as string| || Username to impersonate for the operation. User could be a regular user or a service account in a namespace. | |--as-group strings| || Group to impersonate for the operation, this...
---

|--as string|
||
Username to impersonate for the operation. User could be a regular user or a service account in a namespace.
|
|--as-group strings|
||
Group to impersonate for the operation, this flag can be repeated to specify multiple groups.
|
|--as-uid string|
||
UID to impersonate for the operation.
|
|--as-user-extra strings|
||
User extras to impersonate for the operation, this flag can be repeated to specify multiple values for the same key.
|
|--cache-dir stringDefault: "$HOME/.kube/cache"|
||
Default cache directory
|
|--certificate-authority string|
||
Path to a cert file for the certificate authority
|
|--client-certificate string|
||
Path to a client certificate file for TLS
|
|--client-key string|
||
Path to a client key file for TLS
|
|--cluster string|
||
The name of the kubeconfig cluster to use
|
|--context string|
||
The name of the kubeconfig context to use
|
|--disable-compression|
||
If true, opt-out of response compression for all requests to the server
|
|--insecure-skip-tls-verify|
||
If true, the server's certificate will not be checked for validity. This will make your HTTPS connections insecure
|
|--kubeconfig string|
||
Path to the kubeconfig file to use for CLI requests.
|
|--kuberc string|
||
Path to the kuberc file to use for preferences. This can be disabled by exporting KUBECTL\_KUBERC=false feature gate or turning off the feature KUBERC=off.
|
|--match-server-version|
||
Require server version to match client version
|
|-n, --namespace string|
||
If present, the namespace scope for this CLI request
|
|--password string|
||
Password for basic authentication to the API server
|
|--profile stringDefault: "none"|
||
Name of profile to capture. One of (none|cpu|heap|goroutine|threadcreate|block|mutex|trace)
|
|--profile-output stringDefault: "profile.pprof"|
||
Name of the file to write the profile to
|
|--request-timeout stringDefault: "0"|
||
The length of time to wait before giving up on a single server request. Non-zero values should contain a corresponding time unit (e.g. 1s, 2m, 3h). A value of zero means don't timeout requests.
|
|-s, --server string|
||