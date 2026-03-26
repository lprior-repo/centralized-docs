---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#9-standard
chunk_level: standard
chunk_type: prose
heading: Enable Kubernetes Apiserver flags
token_count: 507
summary: #### Warning: Do **not** reuse a CA that is used in a different context unless you understand the risks and the mechanisms to protect the CA's usage. If you are not running kube-proxy on a host...
---

#### Warning:
Do **not** reuse a CA that is used in a different context unless you understand
the risks and the mechanisms to protect the CA's usage.
If you are not running kube-proxy on a host running the API server,
then you must make sure that the system is enabled with the following
`kube-apiserver` flag:
```
`--enable-aggregator-routing=true
`
```
### Register APIService objects
You can dynamically configure what client requests are proxied to extension
apiserver. The following is an example registration:
```
`
apiVersion: apiregistration.k8s.io/v1
kind: APIService
metadata:
name: &lt;name of the registration object&gt;
spec:
group: &lt;API group name this extension apiserver hosts&gt;
version: &lt;API version this extension apiserver hosts&gt;
groupPriorityMinimum: &lt;priority this APIService for this group, see API documentation&gt;
versionPriority: &lt;prioritizes ordering of this version within a group, see API documentation&gt;
service:
namespace: &lt;namespace of the extension apiserver service&gt;
name: &lt;name of the extension apiserver service&gt;
caBundle: &lt;pem encoded ca cert that signs the server cert used by the webhook&gt;
`
```
The name of an APIService object must be a valid
[path segment name](/docs/concepts/overview/working-with-objects/names/#path-segment-names).
#### Contacting the extension apiserver
Once the Kubernetes apiserver has determined a request should be sent to an extension apiserver,
it needs to know how to contact it.
The `service` stanza is a reference to the service for an extension apiserver.
The service namespace and name are required. The port is optional and defaults to 443.
Here is an example of an extension apiserver that is configured to be called on port "1234",
and to verify the TLS connection against the ServerName
`my-service-name.my-service-namespace.svc` using a custom CA bundle.
```
`apiVersion: apiregistration.k8s.io/v1
kind: APIService
...
spec:
...
service:
namespace: my-service-namespace
name: my-service-name
port: 1234
caBundle: "Ci0tLS0tQk...&lt;base64-encoded PEM bundle&gt;...tLS0K"
...
`
```