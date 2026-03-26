---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#8-standard
chunk_level: standard
chunk_type: prose
heading: Use cases for Kubernetes service accounts
token_count: 472
summary: #### Note: For applications running outside your Kubernetes cluster, you might be considering creating a long-lived ServiceAccount token that is stored in a Secret. This allows authentication, but...
---

#### Note:
For applications running outside your Kubernetes cluster, you might be considering
creating a long-lived ServiceAccount token that is stored in a Secret. This allows authentication, but the Kubernetes project recommends you avoid this approach.
Long-lived bearer tokens represent a security risk as, once disclosed, the token
can be misused. Instead, consider using an alternative. For example, your external
application can authenticate using a well-protected private key `and` a certificate,
or using a custom mechanism such as an [authentication webhook](/docs/reference/access-authn-authz/authentication/#webhook-token-authentication) that you implement yourself.
You can also use TokenRequest to obtain short-lived tokens for your external application.
### Restricting access to Secrets (deprecated)
FEATURE STATE:
`Kubernetes v1.32 [deprecated]`
#### Note:
`kubernetes.io/enforce-mountable-secrets` is deprecated since Kubernetes v1.32. Use separate namespaces to isolate access to mounted secrets.
Kubernetes provides an annotation called `kubernetes.io/enforce-mountable-secrets`
that you can add to your ServiceAccounts. When this annotation is applied,
the ServiceAccount's secrets can only be mounted on specified types of resources,
enhancing the security posture of your cluster.
You can add the annotation to a ServiceAccount using a manifest:
```
`apiVersion: v1
kind: ServiceAccount
metadata:
annotations:
kubernetes.io/enforce-mountable-secrets: "true"
name: my-serviceaccount
namespace: my-namespace
`
```
When this annotation is set to "true", the Kubernetes control plane ensures that
the Secrets from this ServiceAccount are subject to certain mounting restrictions.
1. The name of each Secret that is mounted as a volume in a Pod must appear in the `secrets` field of the
Pod's ServiceAccount.
2. The name of each Secret referenced using `envFrom` in a Pod must also appear in the `secrets`
field of the Pod's ServiceAccount.
3. The name of each Secret referenced using `imagePullSecrets` in a Pod must also appear in the `secrets`
field of the Pod's ServiceAccount.
By understanding and enforcing these restrictions, cluster administrators can maintain a tighter security profile and ensure that secrets are accessed only by the appropriate resources.