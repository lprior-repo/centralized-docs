---
doc_id: ref/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods.md/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods
chunk_id: ref/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods.md/docs-tasks-administer-cluster-guaranteed-scheduling-critical-addon-pods#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 101
summary: and becomes pending (for example when the cluster is highly utilized and either there are other pending pods that schedule into the space vacated by the evicted critical add-on pod or the amount of...
---

and becomes pending (for example when the cluster is highly utilized and either there are other pending pods that schedule into the space
vacated by the evicted critical add-on pod or the amount of resources available on the node changed for some other reason).
Note that marking a pod as critical is not meant to prevent evictions entirely; it only prevents the pod from becoming permanently unavailable.
A static pod marked as critical can't be evicted. However, non-static pods marked as critical are always rescheduled.