---
doc_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts
chunk_id: ref/docs-concepts-security-service-accounts.md/docs-concepts-security-service-accounts#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Authenticating service account credentials
token_count: 944
summary: ### Restricting access to Secrets (deprecated) FEATURE STATE: `Kubernetes v1.32 [deprecated]` #### Note: `kubernetes.io/enforce-mountable-secrets` is deprecated since Kubernetes v1.32. Use separate...
---

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
## Authenticating service account credentials
ServiceAccounts use signed
[JSON Web Tokens](https://www.rfc-editor.org/rfc/rfc7519) (JWTs)
to authenticate to the Kubernetes API server, and to any other system where a
trust relationship exists. Depending on how the token was issued
(either time-limited using a `TokenRequest` or using a legacy mechanism with
a Secret), a ServiceAccount token might also have an expiry time, an audience,
and a time after which the token *starts* being valid. When a client that is
acting as a ServiceAccount tries to communicate with the Kubernetes API server,
the client includes an `Authorization: Bearer &lt;token&gt;` header with the HTTP
request. The API server checks the validity of that bearer token as follows:
1. Checks the token signature.
2. Checks whether the token has expired.
3. Checks whether object references in the token claims are currently valid.
4. Checks whether the token is currently valid.
5. Checks the audience claims.
The TokenRequest API produces *bound tokens* for a ServiceAccount. This
binding is linked to the lifetime of the client, such as a Pod, that is acting
as that ServiceAccount. See [Token Volume Projection](/docs/tasks/configure-pod-container/configure-service-account/#serviceaccount-token-volume-projection)
for an example of a bound pod service account token's JWT schema and payload.
For tokens issued using the `TokenRequest` API, the API server also checks that
the specific object reference that is using the ServiceAccount still exists,
matching by the [unique ID](/docs/concepts/overview/working-with-objects/names) of that
object. For legacy tokens that are mounted as Secrets in Pods, the API server
checks the token against the Secret.
For more information about the authentication process, refer to
[Authentication](/docs/reference/access-authn-authz/authentication/#service-account-tokens).
### Authenticating service account credentials in your own code
If you have services of your own that need to validate Kubernetes service
account credentials, you can use the following methods:
* [TokenReview API](/docs/reference/kubernetes-api/authentication-resources/token-review-v1/)
(recommended)
* OIDC discovery
The Kubernetes project recommends that you use the TokenReview API, because
this method invalidates tokens that are bound to API objects such as Secrets,
ServiceAccounts, Pods or Nodes when those objects are deleted. For example, if you
delete the Pod that contains a projected ServiceAccount token, the cluster
invalidates that token immediately and a TokenReview immediately fails.
If you use OIDC validation instead, your clients continue to treat the token
as valid until the token reaches its expiration timestamp.
Your application should always define the audience that it accepts, and should
check that the token's audiences match the audiences that the application
expects. This helps to minimize the scope of the token so that it can only be
used in your application and nowhere else.