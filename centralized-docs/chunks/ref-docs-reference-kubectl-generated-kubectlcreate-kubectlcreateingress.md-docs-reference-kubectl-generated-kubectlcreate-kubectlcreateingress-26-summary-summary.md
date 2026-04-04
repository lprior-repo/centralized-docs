---
doc_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress
chunk_id: ref/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress.md/docs-reference-kubectl-generated-kubectlcreate-kubectlcreateingress#26-summary
chunk_level: summary
chunk_type: table
heading: Parent Options Inherited
token_count: 125
summary: server string| || The address and port of the Kubernetes API server | |--storage-driver-buffer-duration durationDefault: 1m0s| || Writes in the storage driver will be buffered for this duration, and...
---

server string|
||
The address and port of the Kubernetes API server
|
|--storage-driver-buffer-duration durationDefault: 1m0s|
||
Writes in the storage driver will be buffered for this duration, and committed to the non memory backends as a single transaction
|
|--storage-driver-db stringDefault: "cadvisor"|
||
database name
|
|--storage-driver-host stringDefault: "localhost:8086"|
||
database host:port
|
|--storage-driver-password stringDefault: "root"|
||
database password
|
|--storage-driver-secure|
||
use secure connection with database
|
|--