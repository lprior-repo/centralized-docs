---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#100-summary
chunk_level: summary
chunk_type: prose
heading: `WebhookMatchCondition`
token_count: 79
summary: * 'groups' is the groups to test for. e.g. ('group1' in request.groups) * 'extra' corresponds to the user.Info.GetExtra() method from the authenticator. * 'uid' is the information about the...
---

* 'groups' is the groups to test for. e.g. ('group1' in request.groups)
* 'extra' corresponds to the user.Info.GetExtra() method from the authenticator.
* 'uid' is the information about the requesting user. e.g. request.uid == '1'
Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
|