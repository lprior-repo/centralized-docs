---
doc_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset
chunk_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset#8-summary
chunk_level: summary
chunk_type: table
heading: Options
token_count: 106
summary: kubectl config set contexts.my-context.cluster my-cluster # Set the client-key-data field in the cluster-admin user using --set-raw-bytes option kubectl config set users.cluster-admin.client-key-data...
---

kubectl config set contexts.my-context.cluster my-cluster
# Set the client-key-data field in the cluster-admin user using --set-raw-bytes option
kubectl config set users.cluster-admin.client-key-data cert\_data\_here --set-raw-bytes=true
`
```
## Options
|-h, --help|
||
help for set
|
|--set-raw-bytes tristate[=true]|
||
When writing a []byte PROPERTY\_VALUE, write the given string directly without base64 decoding.
|