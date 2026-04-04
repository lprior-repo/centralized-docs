---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#50-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 114
summary: || List of request headers to inspect for usernames. X-Remote-User is common. | |--secure-port intDefault: 10259| || The port on which to serve HTTPS with authentication and authorization. If 0,...
---

||
List of request headers to inspect for usernames. X-Remote-User is common.
|
|--secure-port intDefault: 10259|
||
The port on which to serve HTTPS with authentication and authorization. If 0, don't serve HTTPS at all.
|
|--show-hidden-metrics-for-version string|
||
The previous version for which you want to show hidden metrics. Only the previous minor version is meaningful, other values will not be allowed. The format is &lt;major&gt;.&lt;minor&gt;, e.g.: '1.16'