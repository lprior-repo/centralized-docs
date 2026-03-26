---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#46-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 104
summary: [default=false] | |--pod-max-in-unschedulable-pods-duration durationDefault: 5m0s| || DEPRECATED: the maximum time a pod can stay in unschedulablePods. If a pod stays in unschedulablePods for longer...
---

[default=false]
|
|--pod-max-in-unschedulable-pods-duration durationDefault: 5m0s|
||
DEPRECATED: the maximum time a pod can stay in unschedulablePods. If a pod stays in unschedulablePods for longer than this value, the pod will be moved from unschedulablePods to backoffQ or activeQ. This flag is deprecated and will be removed in a future version.
|
|--profilingDefault: true|
||