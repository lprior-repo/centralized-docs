---
doc_id: ref/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview.md/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview
chunk_id: ref/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview.md/docs-reference-kubectl-generated-kubectlalpha-kubectlalphakubercview#4-standard
chunk_level: standard
chunk_type: table
heading: Parent Options Inherited
token_count: 304
summary: 't timeout requests. | |-s, --server string| || The address and port of the Kubernetes API server | |--storage-driver-buffer-duration durationDefault: 1m0s| || Writes in the storage driver will be...
---

't timeout requests.
|
|-s, --server string|
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
|--storage-driver-table stringDefault: "stats"|
||
table name
|
|--storage-driver-user stringDefault: "root"|
||
database username
|
|--tls-server-name string|
||
Server name to use for server certificate validation. If it is not provided, the hostname used to contact the server is used
|
|--token string|
||
Bearer token for authentication to the API server
|
|--user string|
||
The name of the kubeconfig user to use
|
|--username string|
||
Username for basic authentication to the API server
|
|--version version[=true]|
||
--version, --version=raw prints version information and quits; --version=vX.Y.Z... sets the reported version
|
|--warnings-as-errors|
||
Treat warnings received from the server as errors and exit with a non-zero exit code
|