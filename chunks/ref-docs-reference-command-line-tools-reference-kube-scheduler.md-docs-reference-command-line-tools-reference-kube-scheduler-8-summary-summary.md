---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#8-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: 'authorized' responses from the webhook authorizer. | |--authorization-webhook-cache-unauthorized-ttl durationDefault: 10s| || The duration to cache 'unauthorized' responses from the webhook...
---

'authorized' responses from the webhook authorizer.
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