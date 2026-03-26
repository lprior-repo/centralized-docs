---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#17-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 124
summary: # Point to the API server referring the cluster name APISERVER=$(kubectl config view -o jsonpath=\"{.clusters[?(@.name==\\\"$CLUSTER\_NAME\\\")].cluster.server}\") # Create a secret to hold a token for...
---

# Point to the API server referring the cluster name
APISERVER=$(kubectl config view -o jsonpath="{.clusters[?(@.name==\\"$CLUSTER\_NAME\\")].cluster.server}")
# Create a secret to hold a token for the default service account
kubectl apply -f - &lt;&lt;EOF
apiVersion: v1
kind: Secret
metadata:
name: default-token
annotations:
kubernetes.io/service-account.name: default
type: kubernetes.io/service-account-token
EOF
# Wait for the token controller to populate the secret with a token:
while !