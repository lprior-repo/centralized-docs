---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#45-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 120
summary: If the component is not specified, defaults to \"kube\" | |--permit-address-sharing| || If true, SO\_REUSEADDR will be used when binding the port. This allows binding to wildcard IPs like 0.0.0.0 and...
---

If the component is not specified, defaults to "kube"
|
|--permit-address-sharing|
||
If true, SO\_REUSEADDR will be used when binding the port. This allows binding to wildcard IPs like 0.0.0.0 and specific IPs in parallel, and it avoids waiting for the kernel to release sockets in TIME\_WAIT state. [default=false]
|
|--permit-port-sharing|
||
If true, SO\_REUSEPORT will be used when binding the port, which allows more than one instance to bind on the same address and port. [default=false]
|
|--