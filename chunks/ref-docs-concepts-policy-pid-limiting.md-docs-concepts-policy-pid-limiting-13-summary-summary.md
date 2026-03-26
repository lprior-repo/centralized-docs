---
doc_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting
chunk_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting#13-summary
chunk_level: summary
chunk_type: prose
heading: PID based eviction
token_count: 122
summary: node can still get into unstable state by hitting the node PIDs limit. Eviction signal value is calculated periodically and does NOT enforce the limit. PID limiting - per Pod and per Node sets the...
---

node can still get into unstable state by hitting the node PIDs limit.
Eviction signal value is calculated periodically and does NOT enforce the limit.
PID limiting - per Pod and per Node sets the hard limit.
Once the limit is hit, workload will start experiencing failures when trying to get a new PID.
It may or may not lead to rescheduling of a Pod,
depending on how workload reacts on these failures and how liveness and readiness
probes are configured for the Pod. However, if limits were set correctly,
you can guarantee that other Pods workload and system processes will not run out of PIDs