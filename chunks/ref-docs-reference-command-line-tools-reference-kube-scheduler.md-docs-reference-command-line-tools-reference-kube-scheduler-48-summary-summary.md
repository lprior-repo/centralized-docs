---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#48-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 125
summary: | |--requestheader-client-ca-file string| || Root certificate bundle to use to verify client certificates on incoming requests before trusting usernames in headers specified by...
---

|
|--requestheader-client-ca-file string|
||
Root certificate bundle to use to verify client certificates on incoming requests before trusting usernames in headers specified by --requestheader-username-headers. WARNING: generally do not depend on authorization being already done for incoming requests.
|
|--requestheader-extra-headers-prefix stringsDefault: "x-remote-extra-"|
||
List of request header prefixes to inspect. X-Remote-Extra- is suggested.
|
|--requestheader-group-headers stringsDefault: "x-remote-group"|
||
List of request headers to inspect for groups. X-Remote-Group is suggested.
|
|--