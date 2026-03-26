---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#39-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 127
summary: --config. | |--leader-electDefault: true| || Start a leader election client and gain leadership before executing the main loop. Enable this when running replicated components for high availability. |...
---

--config.
|
|--leader-electDefault: true|
||
Start a leader election client and gain leadership before executing the main loop. Enable this when running replicated components for high availability.
|
|--leader-elect-lease-duration durationDefault: 15s|
||
The duration that non-leader candidates will wait after observing a leadership renewal until attempting to acquire leadership of a led but unrenewed leader slot. This is effectively the maximum duration that a leader can be stopped before it is replaced by another candidate. This is only applicable if leader election is enabled.
|
|--leader-elect-renew-deadline durationDefault: 10s|
||