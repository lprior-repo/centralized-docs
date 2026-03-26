---
doc_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset
chunk_id: ref/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset.md/docs-reference-kubectl-generated-kubectlconfig-kubectlconfigset#7-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 124
summary: ` # Set the server field on the my-cluster cluster to https://1.2.3.4 kubectl config set clusters.my-cluster.server https://1.2.3.4 # Set the certificate-authority-data field on the my-cluster...
---

` # Set the server field on the my-cluster cluster to https://1.2.3.4
kubectl config set clusters.my-cluster.server https://1.2.3.4
# Set the certificate-authority-data field on the my-cluster cluster
kubectl config set clusters.my-cluster.certificate-authority-data $(echo "cert\_data\_here" | base64 -i -)
# Set the cluster field in the my-context context to my-cluster
kubectl config set contexts.my-context.cluster my-cluster
# Set the client-key-data field in the cluster-admin user using --set-raw-bytes option