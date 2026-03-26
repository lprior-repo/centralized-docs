---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Before you begin
token_count: 934
summary: ### Directly accessing the REST API kubectl handles locating and authenticating to the API server. If you want to directly access the REST API with an http client like `curl` or `wget`, or a browser,...
---

### Directly accessing the REST API
kubectl handles locating and authenticating to the API server. If you want to directly access the REST API with an http client like
`curl` or `wget`, or a browser, there are multiple ways you can locate and authenticate against the API server:
1. Run kubectl in proxy mode (recommended). This method is recommended, since it uses
the stored API server location and verifies the identity of the API server using a
self-signed certificate. No man-in-the-middle (MITM) attack is possible using this method.
2. Alternatively, you can provide the location and credentials directly to the http client.
This works with client code that is confused by proxies. To protect against man in the
middle attacks, you'll need to import a root cert into your browser.
Using the Go or Python client libraries provides accessing kubectl in proxy mode.
#### Using kubectl proxy
The following command runs kubectl in a mode where it acts as a reverse proxy. It handles
locating the API server and authenticating.
Run it like this:
```
`kubectl proxy --port=8080 &amp;
`
```
See [kubectl proxy](/docs/reference/generated/kubectl/kubectl-commands/#proxy) for more details.
Then you can explore the API with curl, wget, or a browser, like so:
```
`curl http://localhost:8080/api/
`
```
The output is similar to this:
```
`{
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
The above example uses the `--insecure` flag. This leaves it subject to MITM
attacks. When kubectl accesses the cluster it uses a stored root certificate
and client certificates to access the server. (These are installed in the
`\~/.kube` directory). Since cluster certificates are typically self-signed, it
may take special configuration to get your http client to use root
certificate.
On some clusters, the API server does not require authentication; it may serve
on localhost, or be protected by a firewall. There is not a standard
for this. [Controlling Access to the Kubernetes API](/docs/concepts/security/controlling-access/)
describes how you can configure this as a cluster administrator.