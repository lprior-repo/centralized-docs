---
doc_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer
chunk_id: tutorial/docs-tasks-extend-kubernetes-configure-aggregation-layer.md/docs-tasks-extend-kubernetes-configure-aggregation-layer#6-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 817
summary: ### Register APIService objects You can dynamically configure what client requests are proxied to extension apiserver. The following is an example registration: ``` ` apiVersion:...
---

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
## What's next
* [Set up an extension api-server](/docs/tasks/extend-kubernetes/setup-extension-api-server/)
to work with the aggregation layer.
* For a high level overview, see
[Extending the Kubernetes API with the aggregation layer](/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/).
* Learn how to [Extend the Kubernetes API Using Custom Resource Definitions](/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/).
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified December 29, 2023 at 9:47 PM PST: [fix outdated link/anchor (bcc55ae7c9)](https://github.com/kubernetes/website/commit/bcc55ae7c97e725cc9727d69324b77a519cc8fab)
## Related Pages

- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [User Impersonation](docs-reference-access-authn-authz-user-impersonation.md)
- [Extending the Kubernetes API](docs-concepts-extend-kubernetes-api-extension.md)
- [Secrets](docs-concepts-configuration-secret.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)