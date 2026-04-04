---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#6-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: | |--authentication-token-webhook-cache-ttl durationDefault: 10s| || The duration to cache responses from the webhook token authenticator. | |--authentication-tolerate-lookup-failureDefault: true| ||...
---

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
|--