---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#49-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 112
summary: \"x-remote-group\"| || List of request headers to inspect for groups. X-Remote-Group is suggested. | |--requestheader-uid-headers strings| || List of request headers to inspect for UIDs. X-Remote-Uid...
---

"x-remote-group"|
||
List of request headers to inspect for groups. X-Remote-Group is suggested.
|
|--requestheader-uid-headers strings|
||
List of request headers to inspect for UIDs. X-Remote-Uid is suggested. Requires the RemoteRequestHeaderUID feature to be enabled.
|
|--requestheader-username-headers stringsDefault: "x-remote-user"|
||
List of request headers to inspect for usernames. X-Remote-User is common.
|
|--secure-port intDefault: 10259|
||