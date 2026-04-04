---
doc_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler
chunk_id: ref/docs-reference-command-line-tools-reference-kube-scheduler.md/docs-reference-command-line-tools-reference-kube-scheduler#40-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 128
summary: | |--leader-elect-renew-deadline durationDefault: 10s| || The interval between attempts by the acting master to renew a leadership slot before it stops leading. This must be less than the lease...
---

|
|--leader-elect-renew-deadline durationDefault: 10s|
||
The interval between attempts by the acting master to renew a leadership slot before it stops leading. This must be less than the lease duration. This is only applicable if leader election is enabled.
|
|--leader-elect-resource-lock stringDefault: "leases"|
||
The type of resource object that is used for locking during leader election. Supported options are 'leases'.
|
|--leader-elect-resource-name stringDefault: "kube-scheduler"|
||
The name of resource object that is used for locking during leader election.
|
|--leader-elect-resource-namespace stringDefault: