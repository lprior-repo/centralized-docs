---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#10-detailed
chunk_level: detailed
chunk_type: table
heading: Related Pages
token_count: 836
summary: ## `WebhookConnectionInfo` **Appears in:** * [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description| |`type`**[Required]** `string`| Controls how the webhook should...
---

## `WebhookConnectionInfo`
**Appears in:**
* [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description|
|`type`**[Required]**
`string`|
Controls how the webhook should communicate with the server.
Valid values:
* KubeConfigFile: use the file specified in kubeConfigFile to locate the
server.
* InClusterConfig: use the in-cluster configuration to call the
SubjectAccessReview API hosted by kube-apiserver. This mode is not
allowed for kube-apiserver.|
|`kubeConfigFile`**[Required]**
`string`|
Path to KubeConfigFile for connection info
Required, if connectionInfo.Type is KubeConfig
|
## `WebhookMatchCondition`
**Appears in:**
* [WebhookConfiguration](#apiserver-k8s-io-v1beta1-WebhookConfiguration)|Field|Description|
|`expression`**[Required]**
`string`|
expression represents the expression which will be evaluated by CEL. Must evaluate to bool.
CEL expressions have access to the contents of the SubjectAccessReview in v1 version.
If version specified by subjectAccessReviewVersion in the request variable is v1beta1,
the contents would be converted to the v1 version before evaluating the CEL expression.
* 'resourceAttributes' describes information for a resource access request and is unset for non-resource requests. e.g. has(request.resourceAttributes) &amp;&amp; request.resourceAttributes.namespace == 'default'
* 'nonResourceAttributes' describes information for a non-resource access request and is unset for resource requests. e.g. has(request.nonResourceAttributes) &amp;&amp; request.nonResourceAttributes.path == '/healthz'.
* 'user' is the user to test for. e.g. request.user == 'alice'
* 'groups' is the groups to test for. e.g. ('group1' in request.groups)
* 'extra' corresponds to the user.Info.GetExtra() method from the authenticator.
* 'uid' is the information about the requesting user. e.g. request.uid == '1'
Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
|
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
Last modified September 04, 2025 at 5:02 PM PST: [Config API for v1.34 (3557e3070d)](https://github.com/kubernetes/website/commit/3557e3070dcd5659f259e302b53f98adfd9a79f1)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [kube proxy config.v1alpha1](docs-reference-config-api-kube-proxy-config-v1alpha1.md)
- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)