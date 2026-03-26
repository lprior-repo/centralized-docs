---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#16-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 115
summary: `# Check all possible clusters, as your .KUBECONFIG may have multiple contexts: kubectl config view -o jsonpath='{\"Cluster name\\tServer\\n\"}{range...
---

`# Check all possible clusters, as your .KUBECONFIG may have multiple contexts:
kubectl config view -o jsonpath='{"Cluster name\\tServer\\n"}{range .clusters[\*]}{.name}{"\\t"}{.cluster.server}{"\\n"}{end}'
# Select name of cluster you want to interact with from above output:
export CLUSTER\_NAME="some\_server\_name"
# Point to the API server referring the cluster name
APISERVER=$(kubectl config view -o jsonpath="{.clusters[?(