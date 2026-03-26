---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#2-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 512
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