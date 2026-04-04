---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#41-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 126
summary: kube-scheduler\"| || The name of resource object that is used for locking during leader election. | |--leader-elect-resource-namespace stringDefault: \"kube-system\"| || The namespace of resource object...
---

kube-scheduler"|
||
The name of resource object that is used for locking during leader election.
|
|--leader-elect-resource-namespace stringDefault: "kube-system"|
||
The namespace of resource object that is used for locking during leader election.
|
|--leader-elect-retry-period durationDefault: 2s|
||
The duration the clients should wait between attempting acquisition and renewal of a leadership. This is only applicable if leader election is enabled.
|
|--log-flush-frequency durationDefault: 5s|
||
Maximum number of seconds between log flushes
|
|--log-text-info-buffer-size quantity|
||
[Alpha]