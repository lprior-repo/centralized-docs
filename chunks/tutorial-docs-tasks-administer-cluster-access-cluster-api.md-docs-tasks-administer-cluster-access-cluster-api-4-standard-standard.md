---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#4-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 427
summary: #### Without kubectl proxy It is possible to avoid using kubectl proxy by passing an authentication token directly to the API server, like this: Using `grep/cut` approach: ``` `# Check all possible...
---

#### Without kubectl proxy
It is possible to avoid using kubectl proxy by passing an authentication token
directly to the API server, like this:
Using `grep/cut` approach:
```
`# Check all possible clusters, as your .KUBECONFIG may have multiple contexts:
kubectl config view -o jsonpath='{"Cluster name\\tServer\\n"}{range .clusters[\*]}{.name}{"\\t"}{.cluster.server}{"\\n"}{end}'
# Select name of cluster you want to interact with from above output:
export CLUSTER\_NAME="some\_server\_name"
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
The output is similar to this:
```
`{
"kind": "APIVersions",
"versions": [
"v1"
],
"serverAddressByClientCIDRs": [
{
"clientCIDR": "0.0.0.0/0",
"serverAddress": "10.0.1.149:443"
}
]
}
`
```