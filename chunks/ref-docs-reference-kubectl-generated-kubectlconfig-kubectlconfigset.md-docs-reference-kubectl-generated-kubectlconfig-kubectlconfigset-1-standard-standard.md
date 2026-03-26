---
doc_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset
chunk_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset#1-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 336
summary: # kubectl config set Set an individual value in a kubeconfig file ## Synopsis Set an individual value in a kubeconfig file. PROPERTY\_NAME is a dot delimited name where each token represents either...
---

# kubectl config set
Set an individual value in a kubeconfig file
## Synopsis
Set an individual value in a kubeconfig file.
PROPERTY\_NAME is a dot delimited name where each token represents either an attribute name or a map key. Map keys may not contain dots.
PROPERTY\_VALUE is the new value you want to set. Binary fields such as 'certificate-authority-data' expect a base64 encoded string unless the --set-raw-bytes flag is used.
Specifying an attribute name that already exists will merge new fields on top of existing values.
```
`kubectl config set PROPERTY\_NAME PROPERTY\_VALUE
`
```
## Examples
```
` # Set the server field on the my-cluster cluster to https://1.2.3.4
kubectl config set clusters.my-cluster.server https://1.2.3.4
# Set the certificate-authority-data field on the my-cluster cluster
kubectl config set clusters.my-cluster.certificate-authority-data $(echo "cert\_data\_here" | base64 -i -)
# Set the cluster field in the my-context context to my-cluster
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