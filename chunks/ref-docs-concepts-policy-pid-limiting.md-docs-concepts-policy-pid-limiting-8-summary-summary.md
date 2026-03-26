---
doc_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting
chunk_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting#8-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: and their containers. PID limiting is an important sibling to [compute resource](/docs/concepts/configuration/manage-resources-containers/) requests and limits. However, you specify it in a different...
---

and their containers.
PID limiting is an important sibling to [compute
resource](/docs/concepts/configuration/manage-resources-containers/) requests
and limits. However, you specify it in a different way: rather than defining a
Pod's resource limit in the `.spec` for a Pod, you configure the limit as a
setting on the kubelet. Pod-defined PID limits are not currently supported.
#### Caution:
This means that the limit that applies to a Pod may be different depending on
where the Pod is scheduled. To make things simple, it's easiest if all Nodes use
the same PID resource limits and reservations.