---
doc_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates
chunk_id: ref/docs-tasks-administer-cluster-certificates.md/docs-tasks-administer-cluster-certificates#4-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 119
summary: 3. Generate server certificate and key. The argument `--subject-alt-name` sets the possible IPs and DNS names the API server will be accessed with. The `MASTER\_CLUSTER\_IP` is usually the first IP...
---

3. Generate server certificate and key.
The argument `--subject-alt-name` sets the possible IPs and DNS names the API server will
be accessed with. The `MASTER\_CLUSTER\_IP` is usually the first IP from the service CIDR
that is specified as the `--service-cluster-ip-range` argument for both the API server and
the controller manager component. The argument `--days` is used to set the number of days
after which the certificate expires.
The sample below also assumes that you are using `cluster.local` as the default
DNS domain name.