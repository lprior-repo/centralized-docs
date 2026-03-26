---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#28-summary
chunk_level: summary
chunk_type: prose
heading: Uses for Secrets
token_count: 111
summary: * If your cloud-native component needs to authenticate to another application that you know is running within the same Kubernetes cluster, you can use a...
---

* If your cloud-native component needs to authenticate to another application that you
know is running within the same Kubernetes cluster, you can use a
[ServiceAccount](/docs/reference/access-authn-authz/authentication/#service-account-tokens)
and its tokens to identify your client.
* There are third-party tools that you can run, either within or outside your cluster,
that manage sensitive data. For example, a service that Pods access over HTTPS,
that reveals a Secret if the client correctly authenticates (for example, with a ServiceAccount
token).