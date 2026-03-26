---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#2-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1013
summary: |--allow-metric-labels stringToStringDefault: []| || The map from metric-label to value allow-list of this label. The key's format is &lt;MetricName&gt;,&lt;LabelName&gt;. The value's format is...
---

|--allow-metric-labels stringToStringDefault: []|
||
The map from metric-label to value allow-list of this label. The key's format is &lt;MetricName&gt;,&lt;LabelName&gt;. The value's format is &lt;allowed\_value&gt;,&lt;allowed\_value&gt;...e.g. metric1,label1='v1,v2,v3', metric1,label2='v1,v2,v3' metric2,label1='v1,v2,v3'.
|
|--allow-metric-labels-manifest string|
||
The path to the manifest file that contains the allow-list mapping. The format of the file is the same as the flag --allow-metric-labels. Note that the flag --allow-metric-labels will override the manifest file.
|
|--authentication-kubeconfig string|
||
kubeconfig file pointing at the 'core' kubernetes server with enough rights to create tokenreviews.authentication.k8s.io. This is optional. If empty, all token requests are considered to be anonymous and no client CA is looked up in the cluster.
|
|--authentication-skip-lookup|
||
If false, the authentication-kubeconfig will be used to lookup missing authentication configuration from the cluster.
|
|--authentication-token-webhook-cache-ttl durationDefault: 10s|
||
The duration to cache responses from the webhook token authenticator.
|
|--authentication-tolerate-lookup-failureDefault: true|
||
If true, failures to look up missing authentication configuration from the cluster are not considered fatal. Note that this can result in authentication that treats all requests as anonymous.
|
|--authorization-always-allow-paths stringsDefault: "/healthz,/readyz,/livez"|
||
A list of HTTP paths to skip during authorization, i.e. these are authorized without contacting the 'core' kubernetes server.
|
|--authorization-kubeconfig string|
||
kubeconfig file pointing at the 'core' kubernetes server with enough rights to create subjectaccessreviews.authorization.k8s.io. This is optional. If empty, all requests not skipped by authorization are forbidden.
|
|--authorization-webhook-cache-authorized-ttl durationDefault: 10s|
||
The duration to cache 'authorized' responses from the webhook authorizer.
|
|--authorization-webhook-cache-unauthorized-ttl durationDefault: 10s|
||
The duration to cache 'unauthorized' responses from the webhook authorizer.
|
|--bind-address stringDefault: 0.0.0.0|
||
The IP address on which to listen for the --secure-port port. The associated interface(s) must be reachable by the rest of the cluster, and by CLI/web clients. If blank or an unspecified address (0.0.0.0 or ::), all interfaces and IP address families will be used.
|
|--cert-dir string|
||
The directory where the TLS certs are located. If --tls-cert-file and --tls-private-key-file are provided, this flag will be ignored.
|
|--client-ca-file string|
||
If set, any request presenting a client certificate signed by one of the authorities in the client-ca-file is authenticated with an identity corresponding to the CommonName of the client certificate.
|
|--config string|
||
The path to the configuration file.
|
|--contention-profilingDefault: true|
||
DEPRECATED: enable block profiling, if profiling is enabled. This parameter is ignored if a config file is specified in --config.
|
|--disable-http2-serving|
||
If true, HTTP2 serving will be disabled [default=false]
|
|--disabled-metrics strings|
||
This flag provides an escape hatch for misbehaving metrics. You must provide the fully qualified metric name in order to disable it. Disclaimer: disabling metrics is higher in precedence than showing hidden metrics.
|
|--emulated-version strings|
||
The versions different components emulate their capabilities (APIs, features, ...) of.
If set, the component will emulate the behavior of this version instead of the underlying binary version.
Version format could only be major.minor, for example: '--emulated-version=wardle=1.2,kube=1.31'.
Options are: kube=1.32..1.35(default:1.35)
If the component is not specified, defaults to "kube"
|
|--feature-gates colonSeparatedMultimapStringString|
||
Comma-separated list of component:key=value pairs that describe feature gates for alpha/experimental features of different components.
If the component is not specified, defaults to "kube". This flag can be repeatedly invoked. For example: --feature-gates 'wardle:featureA=true,wardle:featureB=false' --feature-gates 'kube:featureC=true'Options are:
kube:APIResponseCompression=true|false (BETA - default=true)
kube:APIServerIdentity=true|false (BETA - default=true)
kube:APIServingWithRoutine=true|false (ALPHA - default=false)