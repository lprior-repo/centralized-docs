---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#5-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 118
summary: --allow-metric-labels will override the manifest file. | |--authentication-kubeconfig string| || kubeconfig file pointing at the 'core' kubernetes server with enough rights to create...
---

--allow-metric-labels will override the manifest file.
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