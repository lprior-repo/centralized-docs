---
doc_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl
chunk_id: ref/docs-reference-kubectl-kubectl.md/docs-reference-kubectl-kubectl#10-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 120
summary: \"profile.pprof\"| ||Name of the file to write the profile to| |--request-timeout stringDefault: \"0\"| ||The length of time to wait before giving up on a single server request. Non-zero values should...
---

"profile.pprof"|
||Name of the file to write the profile to|
|--request-timeout stringDefault: "0"|
||The length of time to wait before giving up on a single server request. Non-zero values should contain a corresponding time unit (e.g. 1s, 2m, 3h). A value of zero means don't timeout requests.|
|-s, --server string|
||The address and port of the Kubernetes API server|
|--skip-headers|
||If true, avoid header prefixes in the log messages|
|--skip-log-headers|