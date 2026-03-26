---
doc_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting
chunk_id: ref/docs-concepts-policy-pid-limiting.md/docs-concepts-policy-pid-limiting#6-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 120
summary: `1000` PIDs to prevent using up that node's overall number of available PIDs. If the admin wants to overcommit PIDs similar to CPU or memory, they may do so as well with some additional risks. Either...
---

`1000`
PIDs to prevent using up that node's overall number of available PIDs. If the
admin wants to overcommit PIDs similar to CPU or memory, they may do so as well
with some additional risks. Either way, a single Pod will not be able to bring
the whole machine down. This kind of resource limiting helps to prevent simple
fork bombs from affecting operation of an entire cluster.
Per-Pod PID limiting allows administrators to protect one Pod from another, but
does not ensure that all Pods scheduled onto that host are unable to impact the node overall.