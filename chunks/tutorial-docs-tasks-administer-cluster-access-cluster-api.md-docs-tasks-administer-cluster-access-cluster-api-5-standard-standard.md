---
doc_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api
chunk_id: tutorial/docs-tasks-administer-cluster-access-cluster-api.md/docs-tasks-administer-cluster-access-cluster-api#5-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 222
summary: The output is similar to this: ``` `{ \"kind\": \"APIVersions\", \"versions\": [ \"v1\" ], \"serverAddressByClientCIDRs\": [ { \"clientCIDR\": \"0.0.0.0/0\", \"serverAddress\": \"10.0.1.149:443\" } ] } ` ``` The above...
---

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