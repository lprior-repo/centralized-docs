---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#99-summary
chunk_level: summary
chunk_type: prose
heading: `WebhookMatchCondition`
token_count: 128
summary: * 'resourceAttributes' describes information for a resource access request and is unset for non-resource requests. e.g. has(request.resourceAttributes) &amp;&amp; request.resourceAttributes.namespace...
---

* 'resourceAttributes' describes information for a resource access request and is unset for non-resource requests. e.g. has(request.resourceAttributes) &amp;&amp; request.resourceAttributes.namespace == 'default'
* 'nonResourceAttributes' describes information for a non-resource access request and is unset for resource requests. e.g. has(request.nonResourceAttributes) &amp;&amp; request.nonResourceAttributes.path == '/healthz'.
* 'user' is the user to test for. e.g. request.user == 'alice'
* 'groups' is the groups to test for. e.g. ('group1' in request.groups)