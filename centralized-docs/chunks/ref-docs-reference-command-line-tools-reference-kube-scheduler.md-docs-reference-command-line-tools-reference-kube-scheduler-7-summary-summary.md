---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#7-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: | || A list of HTTP paths to skip during authorization, i.e. these are authorized without contacting the 'core' kubernetes server. | |--authorization-kubeconfig string| || kubeconfig file pointing at...
---

|
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