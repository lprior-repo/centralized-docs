---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#6-standard
chunk_level: standard
chunk_type: prose
heading: Uses for Secrets
token_count: 361
summary: ### Alternatives to Secrets Rather than using a Secret to protect confidential data, you can pick from alternatives. Here are some of your options: * If your cloud-native component needs to...
---

### Alternatives to Secrets
Rather than using a Secret to protect confidential data, you can pick from alternatives.
Here are some of your options:
* If your cloud-native component needs to authenticate to another application that you
know is running within the same Kubernetes cluster, you can use a
[ServiceAccount](/docs/reference/access-authn-authz/authentication/#service-account-tokens)
and its tokens to identify your client.
* There are third-party tools that you can run, either within or outside your cluster,
that manage sensitive data. For example, a service that Pods access over HTTPS,
that reveals a Secret if the client correctly authenticates (for example, with a ServiceAccount
token).
* For authentication, you can implement a custom signer for X.509 certificates, and use
[CertificateSigningRequests](/docs/reference/access-authn-authz/certificate-signing-requests/)
to let that custom signer issue certificates to Pods that need them.
* You can use a [device plugin](/docs/concepts/extend-kubernetes/compute-storage-net/device-plugins/)
to expose node-local encryption hardware to a specific Pod. For example, you can schedule
trusted Pods onto nodes that provide a Trusted Platform Module, configured out-of-band.
You can also combine two or more of those options, including the option to use Secret objects themselves.
For example: implement (or deploy) an [operator](/docs/concepts/extend-kubernetes/operator/)
that fetches short-lived session tokens from an external service, and then creates Secrets based
on those short-lived session tokens. Pods running in your cluster can make use of the session tokens,
and operator ensures they are valid. This separation means that you can run Pods that are unaware of
the exact mechanisms for issuing and refreshing those session tokens.