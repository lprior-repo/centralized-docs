---
doc_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod
chunk_id: ref/docs-tasks-run-application-force-delete-stateful-set-pod.md/docs-tasks-run-application-force-delete-stateful-set-pod#6-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 67
summary: clustered applications which have a need for a stable network identity and stable storage. These applications often have configuration which relies on an ensemble of a fixed number of members with...
---

clustered applications which have a need for a stable network identity and stable storage.
These applications often have configuration which relies on an ensemble of a fixed number of
members with fixed identities. Having multiple members with the same identity can be disastrous
and may lead to data loss (e.g. split brain scenario in quorum-based systems).