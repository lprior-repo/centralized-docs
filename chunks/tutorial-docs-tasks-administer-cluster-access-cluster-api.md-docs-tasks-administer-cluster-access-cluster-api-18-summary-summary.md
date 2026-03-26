---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#18-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 127
summary: type: kubernetes.io/service-account-token EOF # Wait for the token controller to populate the secret with a token: while ! kubectl describe secret default-token | grep -E '^token' &gt;/dev/null; do...
---

type: kubernetes.io/service-account-token
EOF
# Wait for the token controller to populate the secret with a token:
while ! kubectl describe secret default-token | grep -E '^token' &gt;/dev/null; do
echo "waiting for token..." &gt;&amp;2
sleep 1
done
# Get the token value
TOKEN=$(kubectl get secret default-token -o jsonpath='{.data.token}' | base64 --decode)
# Explore the API with TOKEN
curl -X GET $APISERVER/api --header "Authorization: Bearer $TOKEN" --insecure
`
```